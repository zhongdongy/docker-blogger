use serde::{Deserialize, Serialize};

use super::preamble::JSONPreamble;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub name: String,
    pub preamble: JSONPreamble,
    pub view_path: String,
}

impl Clone for Post {
    fn clone(&self) -> Self {
        Post {
            name: self.name.clone(),
            preamble: self.preamble.clone(),
            view_path: self.view_path.clone()
        }
    }
}
