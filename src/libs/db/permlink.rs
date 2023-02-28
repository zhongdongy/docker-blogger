use crate::models::db::DB;
use crate::models::permlink::PermLink;
use crate::models::post::Post;
use crate::utils::db;
use std::{fs, path};

pub struct PermLinkDB {
    base: db::JsonDBBase,
    data: Vec<PermLink>,
}

impl db::JsonDatabase for PermLinkDB {
    type Item = PermLink;
    type Content = Post;

    fn query_posts(&self, key: String) -> Option<Vec<Self::Content>> {
        for perm in self.data.iter() {
            if perm.permlink == key {
                return Some(vec![perm.post.clone().to_owned()]);
            }
        }
        None
    }

    fn insert_post(&mut self, key: String, post: &mut Self::Content, should_flush: bool) {
        let mut inserted = false;
        self.data.iter_mut().for_each(|t| {
            if t.permlink == key {
                t.post = post.to_owned();
                inserted = true;
            }
        });
        if !inserted {
            self.data.push(Self::Item {
                permlink: key,
                post: post.to_owned(),
            })
        }

        if should_flush {
            self.flush();
        }
    }

    fn count_posts(&self, key: Option<String>) -> usize {
        match key {
            None => self.data.len(),
            Some(k) => {
                let mut count = 0;
                self.data.iter().for_each(|t| {
                    if t.permlink == k {
                        count += 1;
                    }
                });
                count
            }
        }
    }

    fn new() -> Self {
        PermLinkDB {
            base: db::JsonDBBase::new(),
            data: vec![],
        }
    }

    fn load(&mut self, contents: String) {
        let json: DB<PermLink> = serde_json::from_str(&contents).unwrap();
        self.base = db::JsonDBBase {
            version: json.version,
            created_at: json.created_at,
            updated_at: json.updated_at,
        };
        self.data = json.data;
    }

    fn flush(&self) -> bool {
        let db_path = path::Path::new(".").join("db").join("permlink.json");
        if db_path.exists() {
            let output: DB<PermLink> = DB {
                created_at: self.base.created_at.clone(),
                updated_at: self.base.updated_at.clone(),
                version: self.base.version.clone(),
                data: self.data.clone(),
            };

            let db_data = serde_json::to_string_pretty(&output).unwrap();
            fs::write(db_path.to_owned(), db_data).unwrap();

            true
        } else {
            false
        }
    }

    fn remove_posts(&mut self, key: String, posts: &mut Vec<Self::Content>, should_flush: bool) {
        let post_names: Vec<String> = posts.iter().map(|p| p.name.clone()).collect();
        let mut temp_data = vec![];
        for permlink in self.data.iter_mut() {
            if permlink.permlink != key || !post_names.contains(&permlink.post.name) {
                temp_data.push(permlink.to_owned());
            }
        }
        self.data = temp_data;

        if should_flush {
            self.flush();
        }
    }

    fn remove_key(&mut self, key: String, should_flush: bool) {
        let mut permlinks_temp: Vec<PermLink> = vec![];
        for permlink in self.data.iter() {
            if permlink.permlink != key {
                permlinks_temp.push(permlink.clone());
            }
        }

        self.data = permlinks_temp;
        if should_flush {
            self.flush();
        }
    }

    fn has_key(&self, key: String) -> bool {
        for permlink in self.data.iter() {
            if permlink.permlink == key {
                return true;
            }
        }
        false
    }
}
