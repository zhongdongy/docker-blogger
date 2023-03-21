use crate::{
    generate_all,
    libs::db::PermLinkDB,
    resource::load_resource,
    utils::{
        db::{get_database, DatabaseSource, JsonDatabase},
        net::is_port_available,
    },
};
use actix_files::NamedFile;
use actix_web::{
    self, dev,
    error::ErrorNotFound,
    get,
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    web, App, HttpResponse, HttpServer, Responder, Result,
};
use log::debug;
use regex::Regex;
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::{borrow::Cow, fs};

#[get("/admin/cache-reload/")]
async fn reload_cache() -> impl Responder {
    let mut resp_body = String::new();
    match generate_all() {
        Ok(_) => resp_body.push_str("Successfully reloaded cache"),
        Err(e) => resp_body.push_str(format!("Unable to reload: {}", e).as_str()),
    };

    HttpResponse::Ok().body(resp_body)
}

#[get("/static/{file_name:.*}")]
async fn static_file(path: web::Path<String>) -> Result<NamedFile> {
    let file_name = path.into_inner();

    let mut file_path = PathBuf::from("static");
    if MAIN_SEPARATOR != '/' {
        for seg in file_name.split('/').into_iter() {
            file_path.push(seg);
        }
    } else {
        file_path.push(file_name);
    }

    Ok(NamedFile::open(file_path)?)
}

#[get("/post/{post_name:.*}")]
async fn view_post(path: web::Path<String>) -> Result<NamedFile> {
    let mut post_name = path.into_inner();

    if post_name.ends_with('/') {
        post_name = post_name.trim_end_matches('/').to_string();
    }

    if !post_name.ends_with(".html") {
        post_name.push_str(".html");
    }

    let mut file_path = PathBuf::from("cached");
    if MAIN_SEPARATOR != '/' {
        for seg in post_name.split('/').into_iter() {
            file_path.push(seg);
        }
    } else {
        file_path.push(post_name);
    }

    debug!(target: "app::dev", "{}", format!("Trying to match post: {}", file_path.to_string_lossy()));

    Ok(NamedFile::open(file_path)?)
}

#[get("/post/{perm_link:[^/]*}/")]
async fn view_perm_link(path: web::Path<String>) -> Result<NamedFile> {
    let perm_link = path.into_inner();
    let perm_db: PermLinkDB = get_database(DatabaseSource::Permlink);
    let posts = perm_db.query_posts(perm_link.clone());
    if let Some(posts) = posts {
        let post = &posts[0];
        let mut file_path = PathBuf::from("cached");

        if MAIN_SEPARATOR != '/' {
            for seg in post.view_path.clone().split('/').into_iter() {
                file_path.push(seg);
            }
        } else {
            file_path.push(post.view_path.clone());
        }

        debug!(target: "app::dev", "{}", format!("Trying to match post (permlink): {}", file_path.to_string_lossy()));

        Ok(NamedFile::open(file_path)?)
    } else {
        // No perm link matches, try files first
        let mut post_name = perm_link.clone();
        if post_name.ends_with('/') {
            post_name = post_name.trim_end_matches('/').to_string();
        }

        if !post_name.ends_with(".html") {
            post_name.push_str(".html");
        }

        let mut file_path = PathBuf::from("cached");
        if MAIN_SEPARATOR != '/' {
            for seg in post_name.split('/').into_iter() {
                file_path.push(seg);
            }
        } else {
            file_path.push(post_name);
        }

        debug!(target: "app::dev", "{}", format!("Trying to match post (fallback): {}", file_path.to_string_lossy()));

        if let Ok(named_file) = NamedFile::open(file_path) {
            Ok(named_file)
        } else {
            Err(ErrorNotFound(format!(
                "Cannot find permanent link `{}`",
                perm_link
            )))
        }
    }
}

#[get("/")]
async fn index_page() -> Result<NamedFile> {
    let file_path = PathBuf::from("cached").join("_index.html");

    Ok(NamedFile::open(file_path)?)
}

#[get("/privacy-policy/")]
async fn privacy_policy_page() -> Result<NamedFile> {
    let file_path = PathBuf::from("cached").join("privacy-policy.html");

    Ok(NamedFile::open(file_path)?)
}

#[get("/{special:.*}/")]
async fn special_page(path: web::Path<String>) -> Result<NamedFile> {
    let mut special = path.into_inner();
    debug!(target: "app::dev", "{special}");
    // // Check for favicon.ico and manifest.json
    // if ["favicon.ico", "manifest.json"].contains(&special.as_str()) {
    //     let mut file_path = PathBuf::from(".");
    //     file_path.push(special);
    //     debug!(target: "app::dev", "Asset: {}", &file_path.to_str().unwrap());
    //     return Ok(NamedFile::open(file_path)?);
    // }

    let mut file_path = PathBuf::from("cached");

    if !special.ends_with(".html") {
        special.push_str(".html");
    }

    file_path.push(special);

    Ok(NamedFile::open(file_path)?)
}

#[get("/{root_assets:[^/]*}")]
async fn global_assets(path: web::Path<String>) -> Result<NamedFile> {
    let root_assets = path.into_inner();
    // Check for favicon.ico and manifest.json
    if ["favicon.ico", "manifest.json"].contains(&root_assets.as_str()) {
        let mut file_path = PathBuf::from(".");
        file_path.push(root_assets);
        return Ok(NamedFile::open(file_path)?);
    }
    if ["sitemap.xml", "robots.txt"].contains(&root_assets.as_str()) {
        let mut file_path = PathBuf::from("cached");
        file_path.push(root_assets);
        return Ok(NamedFile::open(file_path)?);
    }

    Ok(NamedFile::open(".")?)
}

fn add_custom_error_page<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let file_path = PathBuf::from("cached").join("404.html");
    let content = fs::read_to_string(file_path).unwrap();

    let (req, _) = res.into_parts();

    let resp = HttpResponse::NotFound()
        .insert_header(("Content-Type", "text/html"))
        .body(content)
        .map_into_boxed_body();

    Ok(ErrorHandlerResponse::Response(dev::ServiceResponse::new(
        req,
        resp.map_into_right_body(),
    )))
}

////////////////////////////////////////////////////////////////////////////////
//
// Packed handlers
//
////////////////////////////////////////////////////////////////////////////////

#[get("/static/{file_name:.*}")]
async fn static_file_packed(path: web::Path<String>) -> impl Responder {
    let file_name = path.into_inner();

    if let Some(capture) = Regex::new(r"\.(?P<ext>[^.]+)$")
        .unwrap()
        .captures(&file_name)
    {
        let extension = capture.name("ext").unwrap().as_str();
        let ct = match extension {
            "js" => "application/javascript",
            "css" => "text/css",
            "png" => "image/png",
            _ => "",
        };
        let mut file_path = String::from("static/");
        file_path.push_str(&file_name);
        if let Ok(res) = load_resource(&file_path) {
            return res.into_response(ContentType(ct.parse().unwrap()));
        }
    }

    ErrorNotFound("Not found").into()
}

#[get("/{root_assets:[^/]*}")]
async fn global_assets_packed(path: web::Path<String>) -> impl Responder {
    let root_assets = path.into_inner();
    // Check for favicon.ico and manifest.json
    if ["favicon.ico", "manifest.json"].contains(&root_assets.as_str()) {
        if let Ok(res) = load_resource(&root_assets) {
            return match root_assets.as_str() {
                "favicon.ico" => res.into_response(ContentType("image/x-icon".parse().unwrap())),
                "manifest.json" => {
                    res.into_response(ContentType("application/json".parse().unwrap()))
                }
                _ => ErrorNotFound("Not found").into(),
            };
        }
    }

    if ["sitemap.xml", "robots.txt"].contains(&root_assets.as_str()) {
        let mut file_path = PathBuf::from("cached");
        file_path.push(root_assets.clone());
        if file_path.exists() {
            let mut res = HttpResponse::build(StatusCode::OK);
            let ct = match root_assets.as_str() {
                "sitemap.xml" => "application/xml",
                _ => "text/plain",
            };
            res.insert_header((header::CONTENT_TYPE, ContentType(ct.parse().unwrap())));

            return res.body(fs::read_to_string(file_path).unwrap());
        }
    }

    ErrorNotFound("Not found").into()
}

/// Main function for Actix-Web server
#[actix_web::main]
pub async fn run_server(port: Option<u16>) -> std::io::Result<()> {
    if !is_port_available(port.unwrap_or(8080)) {
        let msg = Cow::from(format!("Port {} is NOT available", port.unwrap_or(8080)));
        return Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, msg));
    }
    #[cfg(feature = "unpacked")]
    {
        HttpServer::new(|| {
            let logger = Logger::default();
            App::new()
                .wrap(logger)
                .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, add_custom_error_page))
                .service(index_page)
                .service(privacy_policy_page)
                .service(reload_cache)
                .service(static_file)
                .service(view_perm_link)
                .service(view_post)
                .service(global_assets)
                .service(special_page)
        })
        .bind(("0.0.0.0", port.unwrap_or(8080)))?
        .run()
        .await?;
    }

    #[cfg(feature = "packed")]
    {
        HttpServer::new(|| {
            let logger = Logger::default();
            App::new()
                .wrap(logger)
                .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, add_custom_error_page))
                .service(index_page)
                .service(privacy_policy_page)
                .service(reload_cache)
                .service(static_file_packed)
                .service(view_perm_link)
                .service(view_post)
                .service(global_assets_packed)
                .service(special_page)
        })
        .bind(("0.0.0.0", port.unwrap_or(8080)))?
        .run()
        .await?;
    }

    Ok(())
}
