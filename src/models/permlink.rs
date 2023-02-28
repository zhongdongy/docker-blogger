use serde::{Deserialize, Serialize};

use super::post::Post;

#[derive(Serialize, Deserialize)]
pub struct PermLink {
    pub permlink: String,
    pub post: Post,
}

impl Clone for PermLink {
    fn clone(&self) -> Self {
        Self {
            permlink: self.permlink.clone(),
            post: self.post.clone(),
        }
    }
}
