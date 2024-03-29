use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tera::Error;
use tera::Function;
use tera::Result;
use tera::Value;

#[cfg(feature = "bundled")]
use crate::resource::load_resource;
#[cfg(feature = "bundled")]
use crate::resource::Resource;

pub fn inline_css() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("file") {
            None => Err(Error::msg("No CSS file name provided.")),
            Some(file) => {
                #[cfg(feature = "core")]
                {
                    let css_path = Path::new("static").join("css").join(file.as_str().unwrap());
                    if css_path.is_file() {
                        if let Ok(css_content) = fs::read_to_string(css_path.clone()) {
                            return Ok(Value::String(format!("<style>{}</style>", css_content)));
                        }
                    }
                    println!("{}", css_path.to_str().unwrap());
                    Err(Error::msg(&format!("{} not found", file)))
                }
                #[cfg(feature = "bundled")]
                {
                    if let Ok(Resource::String(css_content)) =
                        load_resource(format!("static/css/{}", file.as_str().unwrap()).as_str())
                    {
                        return Ok(Value::String(format!("<style>{}</style>", css_content)));
                    } else {
                        return Err(Error::msg(&format!("{} not found", file)));
                    }
                }
            }
        }
    })
}
pub fn inline_js() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("file") {
            None => Err(Error::msg("No JS file name provided.")),
            Some(file) => {
                #[cfg(feature = "core")]
                {
                    let js_path = Path::new("static")
                        .join("script")
                        .join(file.as_str().unwrap());
                    if js_path.is_file() {
                        if let Ok(js_content) = fs::read_to_string(js_path) {
                            return Ok(Value::String(format!("<script>{}</script>", js_content)));
                        }
                    }
                    Err(Error::msg(&format!("{} not found", file)))
                }
                #[cfg(feature = "bundled")]
                {
                    if let Ok(Resource::String(js_content)) =
                        load_resource(format!("static/script/{}", file.as_str().unwrap()).as_str())
                    {
                        return Ok(Value::String(format!("<script>{}</script>", js_content)));
                    } else {
                        return Err(Error::msg(&format!("{} not found", file)));
                    }
                }
            }
        }
    })
}

pub fn url_for() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        let route = args.get("route").unwrap();
        let filename = args.get("filename").unwrap();
        match route.as_str().unwrap() {
            "static" => Ok(Value::String(
                format!("/static/{}", filename.as_str().unwrap()).to_string(),
            )),
            _ => Ok(Value::String("/404/".to_string())),
        }
    })
}
