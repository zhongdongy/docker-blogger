use serde::{Deserialize, Serialize};

use super::post::Post;

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub author: String,
    pub posts: Vec<Post>,
}

impl Clone for Author {
    fn clone(&self) -> Self {
        Self {
            author: self.author.clone(),
            posts: self.posts.clone(),
        }
    }
}
