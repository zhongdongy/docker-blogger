use actix_web::{
    body::BoxBody,
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    HttpResponse,
};

#[cfg(all(feature = "core", feature = "bundled"))]
compile_error!("You cannot enable `packed` and `unpacked` features at the same time.");

pub fn load_resource(filepath: &str) -> std::io::Result<Resource> {
    #[cfg(feature = "core")]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Requested resource `{}` doesn't exist", filepath),
        ))
    }

    #[cfg(feature = "bundled")]
    {
        return match filepath {
            "favicon.ico" => Ok(Resource::Bytes(include_bytes!("../favicon.ico"))),
            "static/css/404.css" => Ok(Resource::String(
                include_str!("../static/css/404.css").to_string(),
            )),
            "static/css/base.css" => Ok(Resource::String(
                include_str!("../static/css/base.css").to_string(),
            )),
            "static/css/codehilite.css" => Ok(Resource::String(
                include_str!("../static/css/codehilite.css").to_string(),
            )),
            "static/css/image.css" => Ok(Resource::String(
                include_str!("../static/css/image.css").to_string(),
            )),
            "static/css/text.css" => Ok(Resource::String(
                include_str!("../static/css/text.css").to_string(),
            )),
            "static/css/theme.css" => Ok(Resource::String(
                include_str!("../static/css/theme.css").to_string(),
            )),
            "static/css/util.css" => Ok(Resource::String(
                include_str!("../static/css/util.css").to_string(),
            )),
            "static/img/beian.png" => {
                Ok(Resource::Bytes(include_bytes!("../static/img/beian.png")))
            }
            "static/script/analysis.js" => Ok(Resource::String(
                include_str!("../static/script/analysis.js").to_string(),
            )),
            "static/script/toc.js" => Ok(Resource::String(
                include_str!("../static/script/toc.js").to_string(),
            )),
            "templates/404.jinja2" => Ok(Resource::String(
                include_str!("../templates/404.jinja2").to_string(),
            )),
            "templates/archive_month.jinja2" => Ok(Resource::String(
                include_str!("../templates/archive_month.jinja2").to_string(),
            )),
            "templates/archive_year.jinja2" => Ok(Resource::String(
                include_str!("../templates/archive_year.jinja2").to_string(),
            )),
            "templates/archives.jinja2" => Ok(Resource::String(
                include_str!("../templates/archives.jinja2").to_string(),
            )),
            "templates/base.jinja2" => Ok(Resource::String(
                include_str!("../templates/base.jinja2").to_string(),
            )),
            "templates/blog_page.jinja2" => Ok(Resource::String(
                include_str!("../templates/blog_page.jinja2").to_string(),
            )),
            "templates/home_page.jinja2" => Ok(Resource::String(
                include_str!("../templates/home_page.jinja2").to_string(),
            )),
            "templates/privacy-policy.jinja2" => Ok(Resource::String(
                include_str!("../templates/privacy-policy.jinja2").to_string(),
            )),
            "templates/tag.jinja2" => Ok(Resource::String(
                include_str!("../templates/tag.jinja2").to_string(),
            )),
            "templates/tags.jinja2" => Ok(Resource::String(
                include_str!("../templates/tags.jinja2").to_string(),
            )),
            _ => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Requested resource `{}` doesn't exist", filepath),
                ))
            }
        };
    }
}

pub fn load_template_resource(filepath: &str) -> &'static str {
    return match filepath {
        "templates/404.jinja2" => include_str!("../templates/404.jinja2"),
        "templates/archive_month.jinja2" => include_str!("../templates/archive_month.jinja2"),
        "templates/archive_year.jinja2" => include_str!("../templates/archive_year.jinja2"),
        "templates/archives.jinja2" => include_str!("../templates/archives.jinja2"),
        "templates/base.jinja2" => include_str!("../templates/base.jinja2"),
        "templates/blog_page.jinja2" => include_str!("../templates/blog_page.jinja2"),
        "templates/home_page.jinja2" => include_str!("../templates/home_page.jinja2"),
        "templates/privacy-policy.jinja2" => include_str!("../templates/privacy-policy.jinja2"),
        "templates/tag.jinja2" => include_str!("../templates/tag.jinja2"),
        "templates/tags.jinja2" => include_str!("../templates/tags.jinja2"),
        _ => "",
    };
}
pub enum Resource {
    String(String),
    Bytes(&'static [u8]),
}

impl Resource {
    pub fn into_response(self, ct: ContentType) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::build(StatusCode::OK);

        res.insert_header((header::CONTENT_TYPE, ct.to_string()));

        match self {
            Self::Bytes(b) => res.body(b),
            Self::String(s) => res.body(s),
        }
    }
}
