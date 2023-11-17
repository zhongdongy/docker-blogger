pub mod db;
pub mod markdown;
pub mod parser;
pub mod renderer;
use self::renderer::renderer::extract_toc_and_update_markup;

use super::models::{ArchiveByYear, Author, Blog, PermLink, Post, Tag};
use super::utils::{
    db::{get_database, DatabaseSource, JsonDatabase},
    error::FsError,
};
use chrono::Utc;
use db::{ArchiveDB, AuthorDB, PermLinkDB, TagDB};
use regex::Regex;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use urlencoding;
use walkdir::WalkDir;

use crate::libs::renderer::renderer::render_index_template;
use crate::libs::renderer::{render_content_template, IndexDescriptor};
use crate::models::context::Context;
use crate::models::sitemap;
use crate::models::Month;
use crate::utils::config::get_config;
use crate::utils::sitemap::generate_sitemap;

pub fn build_all(
    blog_path: Option<PathBuf>,
    cache_path: Option<PathBuf>,
) -> Result<bool, Box<dyn Error>> {
    let blog_dir = blog_path.clone().unwrap_or(Path::new(".").join("blogs"));
    let cache_dir = cache_path.clone().unwrap_or(Path::new(".").join("cached"));

    let db_dir = cache_path.clone().unwrap_or(Path::new(".").join("db"));

    // Delete all generated contents
    if cache_dir.exists() {
        fs::remove_dir_all(cache_dir.clone()).unwrap();
    }
    if db_dir.exists() {
        fs::remove_dir_all(db_dir.clone()).unwrap();
    }

    if !blog_dir.exists() {
        return Err(Box::new(FsError::new("Blog directory doesn't exist")));
    }

    // Traverse and collect all blog posts
    let mut blog_files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(blog_dir.clone())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.metadata()?.is_file() && entry.file_name().to_string_lossy().ends_with(".md") {
            blog_files.push(entry.path().to_path_buf());
        }
    }

    // Remove home blog and handle it separately.
    let config = get_config(None);
    let home_post_name = config.public.home_post.unwrap();
    let home_post_file = PathBuf::new()
        .join(blog_dir.clone())
        .join(format!("{}.md", home_post_name));

    let mut blogs: Vec<Blog> = vec![];

    let blog_dir_replace_string = blog_dir.clone().to_string_lossy().to_string();
    let blog_dir_str = blog_dir_replace_string.as_str();

    let mut sitemap_locs: Vec<sitemap::SitemapLoc> = vec![];
    // A regular expression to remove trailing ".html".
    let re = Regex::new(r"\.html$").unwrap();

    let g = Context::from_config(get_config(None));

    for blog_file in blog_files {
        let raw = fs::read_to_string(blog_file.clone()).unwrap();
        let mut cache_file = blog_file.to_string_lossy().to_string();
        cache_file = cache_file.replace(blog_dir_str, "");
        let mut target = cache_dir.join(
            cache_file
                .clone()
                .as_str()
                .replacen("/", "", 1)
                .replacen("\\", "", 1),
        );
        target.set_extension("html");

        let mut view_file = blog_file.clone();
        view_file.set_extension("html");

        let doc_path = re
            .replace_all(
                &view_file
                    .to_string_lossy()
                    .replace(blog_dir_str, "")
                    .replace("\\", "/")
                    .replacen("/", "", 1),
                "",
            )
            .to_string()
            .split("/")
            .map(|seg| urlencoding::encode(seg).to_string())
            .collect::<Vec<String>>()
            .join("/");

        match render_content_template(renderer::TemplateType::Blog, &raw, Some(doc_path)) {
            Ok((preamble, markdown_content, mut html)) => {
                html = extract_toc_and_update_markup(&preamble, markdown_content.as_str(), html);

                let blog = Blog {
                    raw: raw,
                    preamble: preamble.to_json(),
                    markdown: markdown_content,
                    html: html.clone(),
                    source: blog_file.to_string_lossy().to_string(),
                    target: target.to_string_lossy().to_string(),
                    view_path: view_file
                        .to_string_lossy()
                        .replace(blog_dir_str, "")
                        .replace("\\", "/")
                        .replacen("/", "", 1),
                };
                fs::create_dir_all(target.parent().unwrap()).unwrap();
                fs::write(target, html).unwrap();

                // Create sitemap loc entry info
                let _name = re.replace_all(&blog.view_path.clone(), "").to_string();

                let sitemap_loc = sitemap::SitemapLoc::new(
                    format!(
                        "{}/post/{}/",
                        g.site_home,
                        _name
                            .split("/")
                            .map(|seg| { urlencoding::encode(seg).to_string() })
                            .collect::<Vec<String>>()
                            .join("/"),
                    ),
                    blog.preamble.updated_at.clone().unwrap(),
                );
                sitemap_locs.push(sitemap_loc);

                blogs.push(blog);
            }
            Err(e) => {
                eprintln!(
                    "Unable to render page {}: {}",
                    blog_file.to_str().unwrap(),
                    e
                );
            }
        }
    }

    // Generate robots.txt
    let context = Context::from_config(get_config(None));
    fs::write(
        cache_dir.join("robots.txt"),
        format!(
            "User-agent: Googlebot\nDisallow: /nogooglebot/\n\nUser-agent: *\nAllow: /\nSitemap: {}/sitemap.xml",
            context.site_home
        ),
    )
    .unwrap();

    // Generate index db files.
    let _home_post_file_string = home_post_file.to_str().unwrap().to_string();
    let mut temp_blogs: Vec<Blog> = vec![];
    blogs.iter().for_each(|b| {
        if b.source != _home_post_file_string {
            temp_blogs.push(b.clone());
        }
    });

    generate_db(temp_blogs).unwrap();

    // Generate home post
    let home_raw = fs::read_to_string(home_post_file.clone()).unwrap();
    if let Ok((preamble, markdown_content, mut html)) =
        render_content_template(renderer::TemplateType::Home, &home_raw, None)
    {
        html = extract_toc_and_update_markup(&preamble, markdown_content.as_str(), html);
        let target = cache_dir.join("_index.html");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();
    }

    // Generate tags page
    if let Ok(html) = render_index_template(renderer::TemplateType::PageTags, None) {
        let target = cache_dir.join("tags.html");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();

        // Add tags page to sitemap
        let mut loc_tags = sitemap::SitemapLoc::new(
            format!("{}/tags/", g.site_home),
            Utc::now().date_naive().format("%Y-%m-%d").to_string(),
        );
        loc_tags.changefreq = String::from("weekly");
        sitemap_locs.push(loc_tags);
    }

    // Generate tag pages
    let tag_db: TagDB = get_database(DatabaseSource::Tag);
    tag_db.data().iter().for_each(|t| {
        let tag_file = PathBuf::new().join(t.tag.clone());

        if let Ok(html) = render_index_template(
            renderer::TemplateType::PageTag,
            Some(IndexDescriptor::Tag(t.tag.to_owned())),
        ) {
            let cache_file = tag_file.to_string_lossy().to_string();
            let mut target = cache_dir.join("tag").join(
                cache_file
                    .clone()
                    .as_str()
                    .replacen("/", "", 1)
                    .replacen("\\", "", 1),
            );
            target.set_extension("html");

            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::write(target, html).unwrap();

            // Add tag page to sitemap
            let loc_tag = sitemap::SitemapLoc::new(
                format!(
                    "{}/tag/{}/",
                    g.site_home,
                    urlencoding::encode(&t.tag).to_string()
                ),
                Utc::now().date_naive().format("%Y-%m-%d").to_string(),
            );
            sitemap_locs.push(loc_tag);
        }
    });

    // Generate archives page
    if let Ok(html) = render_index_template(renderer::TemplateType::PageArchives, None) {
        let target = cache_dir.join("archives.html");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();

        // Add archives page to sitemap
        let loc_archives = sitemap::SitemapLoc::new(
            format!("{}/archives/", g.site_home),
            Utc::now().date_naive().format("%Y-%m-%d").to_string(),
        );
        sitemap_locs.push(loc_archives);
    }

    // Generate archive (year) pages
    let arv_db: ArchiveDB = get_database(DatabaseSource::Archive);
    arv_db.archives().iter().for_each(|arv_year| {
        if let Ok(html) = render_index_template(
            renderer::TemplateType::PageArchivesYear,
            Some(IndexDescriptor::ArchiveYear(arv_year.year.clone())),
        ) {
            let target = cache_dir.join(format!("archives/{}.html", arv_year.year));

            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::write(target, html).unwrap();

            // Add archive (year) page to sitemap
            let mut loc_archives_year = sitemap::SitemapLoc::new(
                format!("{}/archives/{}/", g.site_home, arv_year.year),
                Utc::now().date_naive().format("%Y-%m-%d").to_string(),
            );
            loc_archives_year.changefreq = String::from("weekly");
            sitemap_locs.push(loc_archives_year);
        }

        // Generate archive (year/month) pages
        arv_year.months.iter().for_each(|arv_month| {
            let month = <Month as Into<String>>::into(arv_month.month);
            if let Ok(html) = render_index_template(
                renderer::TemplateType::PageArchivesMonth,
                Some(IndexDescriptor::ArchiveMonth((
                    arv_year.year.clone(),
                    month.clone(),
                ))),
            ) {
                let target = cache_dir.join(format!("archives/{}/{}.html", arv_year.year, month));

                fs::create_dir_all(target.parent().unwrap()).unwrap();
                fs::write(target, html).unwrap();

                // Add archive (month) page to sitemap
                let mut loc_archives_month = sitemap::SitemapLoc::new(
                    format!(
                        "{}/archives/{}/{}/",
                        g.site_home,
                        arv_year.year,
                        <Month as Into<String>>::into(arv_month.month)
                    ),
                    Utc::now().date_naive().format("%Y-%m-%d").to_string(),
                );
                loc_archives_month.changefreq = String::from("weekly");
                sitemap_locs.push(loc_archives_month);
            }
        });
    });

    // Generate 404 and privacy-policy pages
    if let Ok(html) = render_index_template(renderer::TemplateType::PageNotFound, None) {
        let target = cache_dir.join("404.html");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();
    }
    if let Ok(html) = render_index_template(renderer::TemplateType::PagePrivacyPolicy, None) {
        let target = cache_dir.join("privacy-policy.html");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, html).unwrap();
    }

    // Generate sitemap
    if let Ok(sitemap_content) = generate_sitemap(&sitemap_locs) {
        let target = cache_dir.join("sitemap.xml");

        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, sitemap_content).unwrap();
    }

    Ok(true)
}

pub fn generate_db(blogs: Vec<Blog>) -> Result<bool, Box<dyn Error>> {
    let mut db_tag = get_database::<TagDB, Tag>(DatabaseSource::Tag);
    let mut db_permlink = get_database::<PermLinkDB, PermLink>(DatabaseSource::Permlink);
    let mut db_author = get_database::<AuthorDB, Author>(DatabaseSource::Author);
    let mut db_archive = get_database::<ArchiveDB, ArchiveByYear>(DatabaseSource::Archive);
    let mut temp_posts = vec![];
    let mut owned_post: Post;

    for blog in blogs {
        let preamble = blog.preamble.clone();
        // Build tag database
        let tags = preamble.tags.clone().unwrap_or(vec![]);
        let re = Regex::new(r"\.html$").unwrap();
        let _name = re.replace_all(&blog.view_path.clone(), "").to_string();

        let post = Post {
            name: _name,
            preamble: preamble.clone(),
            view_path: blog.view_path.clone(),
        };
        for tag in tags {
            db_tag.insert_post(tag, &mut post.clone(), false)
        }
        db_author.insert_post(preamble.author.clone(), &mut post.clone(), false);

        if let Some(perm) = preamble.permanent_link {
            db_permlink.insert_post(perm, &mut post.clone(), false);
        }
        owned_post = post.clone().to_owned();
        temp_posts.push(owned_post);
    }

    db_tag.flush();
    db_author.flush();
    db_permlink.flush();
    db_archive.insert_posts(&mut temp_posts, true);

    Ok(true)
}
