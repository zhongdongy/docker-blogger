use crate::models::preamble::JSONPreamble;
use crate::models::preamble::Preamble;
use regex::Regex;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

use crate::utils::error;
use chrono::{NaiveDate, Utc};
use serde_yaml;

pub fn parse_document(content: &str) -> Result<(Preamble, String), Box<dyn Error>> {
    let re = Regex::new(r"^---([\s\S]+)---").unwrap();
    if let Some(captures) = re.captures(content.trim()) {
        if let Some(preamble_raw_content) = captures.get(0) {
            let mut preamble_content = preamble_raw_content.as_str();
            if preamble_content.contains("---") {
                let split: Vec<&str> = preamble_content.split("---").collect();
                if split.len() > 0 {
                    preamble_content = split[1];
                }
            }
            let mut markdown_bare = content.replace(preamble_content, "");
            markdown_bare = markdown_bare.trim().trim_start_matches("---").to_string();

            // Handle math equations in LaTeX
            markdown_bare = markdown_bare.replace("\\", "\\\\");

            return match parse_preamble(preamble_content) {
                Ok(preamble) => Ok((preamble, markdown_bare)),
                Err(e) => Err(Box::new(e)),
            };
        }
    }
    return Err(Box::new(error::ParserError::new("No preamble found")));
}

type PreambleErrorInfo = (String, String);
pub struct PreambleError {
    errors: Vec<PreambleErrorInfo>,
}

impl PreambleError {
    pub fn record(&mut self, part: String, issue: String) {
        self.errors.push((part, issue));
    }

    pub fn new() -> Self {
        Self { errors: vec![] }
    }
}

impl Error for PreambleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for PreambleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "{}",
            self.errors
                .iter()
                .map(|e| { format!("{}: {}", e.0, e.1) })
                .collect::<Vec<String>>()
                .join("\n")
        )
        .unwrap())
    }
}

impl Debug for PreambleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub fn parse_preamble(content: &str) -> Result<Preamble, PreambleError> {
    if let Ok(_preamble) = serde_yaml::from_str::<JSONPreamble>(content) {
        let mut preamble = Preamble {
            author: _preamble.author,
            author_avatar: _preamble.author_avatar,
            author_email: _preamble.author_email,
            title: _preamble.title,
            keywords: _preamble.keywords.clone(),
            description: _preamble.description,
            created_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
            tags: _preamble.tags.clone(),
            permanent_link: _preamble.permanent_link,
            renderer_params: _preamble.renderer_params.clone(),
            redirect: _preamble.redirect,
        };

        if let None = _preamble.renderer_params.clone() {
            preamble.renderer_params = Some(vec![]);
        }
        if let None = _preamble.tags.clone() {
            preamble.tags = Some(vec![]);
        }
        if let None = _preamble.keywords.clone() {
            preamble.keywords = Some(vec![]);
        }

        if let Some(c_at) = _preamble.created_at {
            preamble.created_at = NaiveDate::parse_from_str(c_at.as_str(), "%Y-%m-%d").unwrap();
        }
        if let Some(u_at) = _preamble.updated_at {
            preamble.updated_at = NaiveDate::parse_from_str(u_at.as_str(), "%Y-%m-%d").unwrap();
        }
        Ok(preamble)
    } else {
        let mut err = PreambleError::new();
        err.record(
            String::from("parser"),
            format!("Unable to parse `{}`", content),
        );
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_document, parse_preamble};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_preamble() {
        let raw_content = "title: 初心\nauthor: 阿东\nkeywords:\n- 随笔\n- 感悟\ndescription: 在低落之时凝聚前进的动力，在苦难之时探求前进的方向\nauthor_email: zhongdong_y@outlook.com\ncreated_at: \"2023-02-05\"\nupdated_at: \"2023-02-05\"\ntags:\n- 随笔\npermanent_link: to-a-life-long-journey\nrenderer_params: \n- enable-toc\n- content-serif\n- content-justify";

        let preamble = parse_preamble(raw_content).unwrap();
        assert_eq!(
            preamble.created_at,
            NaiveDate::parse_from_str("2023-02-05", "%Y-%m-%d").unwrap()
        );
    }

    #[test]
    fn test_parse_document() {
        let raw_content = "---\ntitle: 初心\nauthor: 阿东\nkeywords:\n- 随笔\n- 感悟\ndescription: 在低落之时凝聚前进的动力，在苦难之时探求前进的方向\nauthor_email: zhongdong_y@outlook.com\ncreated_at: \"2023-02-05\"\nupdated_at: \"2023-02-05\"\ntags:\n- 随笔\npermanent_link: to-a-life-long-journey\nrenderer_params: \n- enable-toc\n- content-serif\n- content-justify\n---\n\n### 初心是什么？\n---\n你好";
        match parse_document(raw_content) {
            Ok((preamble, content)) => {
                assert_eq!(content, "\n\n### 初心是什么？\n---\n你好");
                assert_eq!(preamble.title, "初心");
            }
            Err(_) => {
                panic!("Should not panic!")
            }
        }

        let raw_content = "\n---\ntitle: 初心\nauthor: 阿东\nkeywords:\n- 随笔\n- 感悟\ndescription: 在低落之时凝聚前进的动力，在苦难之时探求前进的方向\nauthor_email: zhongdong_y@outlook.com\ncreated_at: \"2023-02-05\"\nupdated_at: \"2023-02-05\"\ntags:\n- 随笔\npermanent_link: to-a-life-long-journey\nrenderer_params: \n- enable-toc\n- content-serif\n- content-justify\n---\n\n### 初心是什么？\n";
        match parse_document(raw_content) {
            Ok((preamble, content)) => {
                assert_eq!(content, "\n\n### 初心是什么？");
                assert_eq!(preamble.author, "阿东");
            }
            Err(_) => {
                panic!("Should not panic!")
            }
        }
    }
}
