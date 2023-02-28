use std::collections::HashMap;
use std::fs;
use tera::Error;
use tera::Function;
use tera::Result;
use tera::Value;

use std::path::Path;

pub fn inline_css() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("file") {
            None => Err(Error::msg("No CSS file name provided.")),
            Some(file) => {
                let css_path = Path::new("static").join("css").join(file.as_str().unwrap());
                if css_path.is_file() {
                    if let Ok(css_content) = fs::read_to_string(css_path.clone()) {
                        return Ok(Value::String(format!("<style>{}</style>", css_content)));
                    }
                }
                println!("{}", css_path.to_str().unwrap());
                Err(Error::msg(&format!("{} not found", file)))
            }
        }
    })
}
pub fn inline_js() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("file") {
            None => Err(Error::msg("No JS file name provided.")),
            Some(file) => {
                let js_path = Path::new("static")
                    .join("script")
                    .join(file.as_str().unwrap());
                if js_path.is_file() {
                    if let Ok(js_content) = fs::read_to_string(js_path) {
                        return Ok(Value::String(format!("<style>{}</style>", js_content)));
                    }
                }
                Err(Error::msg(&format!("{} not found", file)))
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
