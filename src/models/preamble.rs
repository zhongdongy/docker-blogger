use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Preamble {
    pub author: String,
    pub author_avatar: Option<String>,
    pub author_email: Option<String>,
    pub title: String,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub tags: Option<Vec<String>>,
    pub permanent_link: Option<String>,
    pub renderer_params: Option<Vec<String>>,
    pub redirect: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONPreamble {
    pub author: String,
    pub author_avatar: Option<String>,
    pub author_email: Option<String>,
    pub title: String,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub tags: Option<Vec<String>>,
    pub permanent_link: Option<String>,
    pub renderer_params: Option<Vec<String>>,
    pub redirect: Option<String>,
}

impl Clone for JSONPreamble {
    fn clone(&self) -> Self {
        JSONPreamble {
            author: self.author.clone(),
            author_avatar: self.author_avatar.clone(),
            author_email: self.author_email.clone(),
            title: self.title.clone(),
            keywords: self.keywords.clone(),
            description: self.description.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            tags: self.tags.clone(),
            permanent_link: self.permanent_link.clone(),
            renderer_params: self.renderer_params.clone(),
            redirect: self.redirect.clone(),
        }
    }
}

impl Preamble {
    pub fn to_json(&self) -> JSONPreamble {
        JSONPreamble {
            author: self.author.to_string(),
            author_avatar: self.author_avatar.clone(),
            author_email: self.author_email.clone(),
            title: self.title.to_string(),
            keywords: self.keywords.clone(),
            description: self.description.clone(),
            created_at: Some(self.created_at.format("%Y-%m-%d").to_string()),
            updated_at: Some(self.updated_at.format("%Y-%m-%d").to_string()),
            tags: self.tags.clone(),
            permanent_link: self.permanent_link.clone(),
            renderer_params: self.renderer_params.clone(),
            redirect: self.redirect.clone(),
        }
    }
}
