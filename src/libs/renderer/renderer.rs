use crate::libs::db::ArchiveDB;
use crate::libs::db::TagDB;
use crate::libs::parser::parse_toc;
use crate::models::context::Context as SiteContext;
use crate::utils::avatar::get_gravatar_url;
use crate::utils::config::get_config;
use crate::utils::db::get_database;
use crate::utils::db::DatabaseSource;
use crate::utils::db::JsonDatabase;
use crate::utils::error::RendererError;
use std::error::Error;

use crate::libs::markdown::markdown_to_html;
use crate::libs::parser::parse_document;
use pinyin::ToPinyin;
use tera::Context;
use tera::Tera;

use super::func;
use crate::models::preamble::Preamble;

pub enum TemplateType {
    Blog,
    Home,
    PageTag,
    PageTags,
    PageArchives,
    PageArchivesYear,
    PageArchivesMonth,
    PageNotFound,
    PagePrivacyPolicy,
}

pub enum IndexDescriptor {
    Tag(String),
    ArchiveYear(String),
    ArchiveMonth((String, String)),
}

pub fn render_content_template(
    ty: TemplateType,
    raw: &str,
    doc_path: Option<String>,
) -> Result<(Preamble, String, String), Box<dyn Error>> {
    match parse_document(raw) {
        Ok((preamble, markdown_raw)) => {
            match markdown_to_html(&markdown_raw) {
                Ok(markdown_html) => {
                    let tera = load_tera().unwrap();

                    let mut context = load_context(true);
                    context.insert("preamble", &preamble.to_json());
                    context.insert("html", &markdown_html);

                    // Check for LaTeX expressions
                    context.insert("enable_latex", &true);

                    // Composite `avatar_url` property
                    let mut avatar_url =
                        "https://dummyimage.com/80/2196f3/000000/&text=+".to_string();

                    match preamble.author_email.clone() {
                        Some(mail) => avatar_url = get_gravatar_url(&mail),
                        _ => (),
                    };
                    match preamble.author_avatar.clone() {
                        Some(avatar) => avatar_url = avatar,
                        _ => (),
                    };

                    context.insert("avatar_url", &avatar_url);

                    let template_name = match ty {
                        TemplateType::Blog => {
                            if let Some(perm_link) = preamble.permanent_link.clone() {
                                context.insert("link", &format!("/post/{}/", &perm_link));
                            } else {
                                context.insert("link", &format!("/post/{}/", &doc_path.unwrap()))
                            }
                            context.insert("title", &preamble.title);

                            "blog_page.jinja2"
                        }
                        TemplateType::Home => {
                            // Insert site tags to context
                            let tag_db: TagDB = get_database(DatabaseSource::Tag);
                            let mut tags_count_tuples: Vec<(String, usize)> = tag_db
                                .tags_count()
                                .iter()
                                .map(|it| (it.0.to_owned(), it.1.to_owned()))
                                .collect();
                            tags_count_tuples.sort_by(|a, b| b.1.cmp(&a.1));
                            context.insert(
                                "site_tags",
                                if tags_count_tuples.len() >= 10 {
                                    &tags_count_tuples[0..10]
                                } else {
                                    &tags_count_tuples
                                },
                            );

                            context.insert("title", &format!("{}", &preamble.title));
                            context.insert("link", &"/");

                            "home_page.jinja2"
                        }
                        _ => "",
                    };

                    return match tera.render(&template_name, &context) {
                        Ok(res) => Ok((preamble, markdown_raw, res)),
                        Err(e) => {
                            println!("Error: {}", e);
                            let mut cause = e.source();
                            while let Some(e) = cause {
                                println!("Reason: {}", e);
                                cause = e.source();
                            }
                            Err(Box::new(e))
                        }
                    };
                }
                Err(e) => Err(Box::new(RendererError::new(&format!(
                    "Cannot render page: {}",
                    e
                )))),
            }
        }
        Err(e) => Err(Box::new(RendererError::new(&format!(
            "Cannot render page: {}",
            e
        )))),
    }
}

pub fn render_index_template(
    ty: TemplateType,
    descriptor: Option<IndexDescriptor>,
) -> Result<String, Box<dyn Error>> {
    let tera = load_tera().unwrap();

    let mut context = load_context(false);

    let template_name = match ty {
        TemplateType::PageTag => {
            // Insert site tags to context
            let tag_db: TagDB = get_database(DatabaseSource::Tag);
            let mut tags_count_tuples: Vec<(String, usize)> = tag_db
                .tags_count()
                .iter()
                .map(|it| (it.0.to_owned(), it.1.to_owned()))
                .collect();
            tags_count_tuples.sort_by(|a, b| b.1.cmp(&a.1));
            if let Some(desc) = descriptor {
                if let IndexDescriptor::Tag(tag) = desc {
                    let posts = tag_db.query_posts(tag.clone()).unwrap();
                    context.insert("posts", &posts);
                    context.insert("tag_name", &tag);
                    context.insert("link", &format!("/tag/{}/", &tag));
                    context.insert("title", &format!("标签 ({})", &tag));
                }
            }
            context.insert("tags", &tags_count_tuples);
            "tag.jinja2"
        }
        TemplateType::PageTags => {
            // Insert site tags to context
            let tag_db: TagDB = get_database(DatabaseSource::Tag);
            let mut tags_count_tuples: Vec<(String, usize)> = tag_db
                .tags_count()
                .iter()
                .map(|it| (it.0.to_owned(), it.1.to_owned()))
                .collect();
            tags_count_tuples.sort_by(|a, b| b.1.cmp(&a.1));
            context.insert("tags", &tags_count_tuples);

            context.insert("link", &"/tags/");
            context.insert("title", &"所有标签");

            "tags.jinja2"
        }
        TemplateType::PageArchives => {
            let arv_db: ArchiveDB = get_database(DatabaseSource::Archive);
            let archives = arv_db.archives();
            context.insert("archives", &archives);
            context.insert("link", &"/archives/");
            context.insert("title", &"发布历史");

            "archives.jinja2"
        }
        TemplateType::PageArchivesYear => {
            if let Some(IndexDescriptor::ArchiveYear(year)) = descriptor {
                let arv_db: ArchiveDB = get_database(DatabaseSource::Archive);
                let mut posts = arv_db.query_posts(year.clone()).unwrap();
                posts.sort_by(|a, b| b.preamble.created_at.cmp(&a.preamble.created_at));
                context.insert("year", &year);
                context.insert("posts", &posts);
                context.insert("link", &format!("/archives/{}/", &year));
                context.insert("title", &format!("发布历史 ({year})"));

                "archive_year.jinja2"
            } else {
                ""
            }
        }
        TemplateType::PageArchivesMonth => {
            if let Some(IndexDescriptor::ArchiveMonth((year, month))) = descriptor {
                let arv_db: ArchiveDB = get_database(DatabaseSource::Archive);
                let arv_year = arv_db.by_year(year.clone()).unwrap();
                let mut posts = arv_year.posts_by_month(&month);
                posts.sort_by(|a, b| b.preamble.created_at.cmp(&a.preamble.created_at));
                context.insert("year", &year);
                context.insert("month", &format!("{year}-{month}"));
                context.insert("posts", &posts);
                context.insert("link", &format!("/archives/{}/{}/", &year, &month));
                context.insert("title", &format!("发布历史 ({year}-{month})"));

                "archive_month.jinja2"
            } else {
                ""
            }
        }
        TemplateType::PagePrivacyPolicy => {
            context.insert("link", &"/privacy-policy/");
            context.insert("title", &"隐私政策");
            "privacy-policy.jinja2"
        }
        TemplateType::PageNotFound => {
            context.insert("link", &"/404/");
            context.insert("title", &"页面没有找到");
            "404.jinja2"
        }
        _ => "",
    };

    return match tera.render(&template_name, &context) {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
            Err(Box::new(e))
        }
    };
}
fn load_tera() -> Result<Tera, Box<dyn Error>> {
    let mut tera = match Tera::parse("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            return Err(Box::new(e));
        }
    };
    tera.register_function("inline_css", func::inline_css());
    tera.register_function("inline_js", func::inline_js());
    tera.register_function("url_for", func::url_for());
    tera.build_inheritance_chains()?;

    Ok(tera)
}
fn load_context(is_post: bool) -> Context {
    let mut context = Context::new();
    context.insert("is_post", &is_post);

    let g = SiteContext::from_config(get_config(None));
    context.insert("g", &g);

    context
}

pub fn extract_toc_and_update_markup<'a>(
    preamble: &'a Preamble,
    markdown_content: &'a str,
    mut html: String,
) -> String {
    // Handle TOC
    if preamble
        .renderer_params
        .clone()
        .unwrap_or(vec![])
        .contains(&String::from("enable-toc"))
    {
        let mut headings = parse_toc(&markdown_content, Some(3));
        headings.iter_mut().for_each(|h| {
            // Convert Chinese characters to Pinyin form and avoid
            // `Invalid selector` error when using TOC.
            let mut chars: Vec<String> = vec![];
            let ids: &str = h.id.as_str();
            ids.chars().for_each(|c| {
                if let Some(py) = c.to_pinyin() {
                    chars.push(py.plain().to_string());
                } else {
                    let s = c.to_string();
                    chars.push(s);
                };
            });
            h.id = chars.join("");

            html = html.replace(
                &format!("<{}>{}</{}>", h.tag, h.content, h.tag),
                &format!("<{} id='{}'>{}</{}>", h.tag, h.id, h.content, h.tag),
            );
        });

        // Replace TOC symbol in HTML.
        if let Ok(json_str) = serde_json::to_string(&headings) {
            html = html.replace("@{HEADINGS_JSON}", &json_str);
        }
    }
    html
}
