use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DB<T> {
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub data: Vec<T>,
}

impl<T> DB<T> {
    pub fn new() -> Self {
        let version: &str = env!("CARGO_PKG_VERSION");
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            version: String::from(version),
            created_at: now.to_owned(),
            updated_at: now,
            data: vec![],
        }
    }
}
