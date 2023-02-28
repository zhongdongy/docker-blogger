use serde::{Deserialize, Serialize};

use super::post::Post;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub posts: Vec<Post>,
}

impl Clone for Tag {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            posts: self.posts.clone(),
        }
    }
}
