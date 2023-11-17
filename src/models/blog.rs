use serde::{Deserialize, Serialize};

use super::preamble::JSONPreamble;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blog {
    pub preamble: JSONPreamble,
    pub raw: String,
    pub markdown: String,
    pub html: String,
    pub source: String,
    pub target: String,
    pub view_path: String,
}
