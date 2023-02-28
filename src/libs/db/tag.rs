use crate::models::db::DB;
use crate::models::post::Post;
use crate::models::tag::Tag;
use crate::utils::db;
use std::{collections::HashMap, fs, path};

pub struct TagDB {
    base: db::JsonDBBase,
    data: Vec<Tag>,
}

impl TagDB {
    pub fn insert_posts(&mut self, key: String, posts: &mut Vec<Post>, should_flush: bool) {
        let mut inserted = false;
        self.data.iter_mut().for_each(|t| {
            if t.tag == key {
                t.posts.append(&mut posts.to_owned());
                inserted = true;
            }
        });
        if !inserted {
            self.data.push(Tag {
                tag: key,
                posts: posts.to_owned(),
            })
        }

        if should_flush {
            db::JsonDatabase::flush(self);
        }
    }

    pub fn tags(&self) -> Vec<String> {
        self.data.iter().map(|t| t.tag.clone()).collect()
    }

    pub fn tags_count(&self) -> HashMap<String, usize> {
        let mut hashmap = HashMap::new();
        self.data.iter().for_each(|t| {
            hashmap.insert(t.tag.clone(), t.posts.len());
        });

        hashmap
    }
    pub fn data(&self) -> Vec<Tag> {
        self.data.clone()
    }
}

impl db::JsonDatabase for TagDB {
    type Item = Tag;
    type Content = Post;

    fn query_posts(&self, key: String) -> Option<Vec<Self::Content>> {
        let mut res: Vec<Post> = vec![];

        self.data.iter().for_each(|t| {
            if t.tag == key {
                res.append(&mut t.posts.clone().to_owned());
            }
        });

        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }

    fn insert_post(&mut self, key: String, post: &mut Self::Content, should_flush: bool) {
        let mut inserted = false;
        self.data.iter_mut().for_each(|t| {
            if t.tag == key {
                t.posts.push(post.to_owned());
                inserted = true;
            }
        });
        if !inserted {
            self.data.push(Self::Item {
                tag: key,
                posts: vec![post.to_owned()],
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
                    if t.tag == k {
                        count += 1;
                    }
                });
                count
            }
        }
    }

    fn new() -> Self {
        TagDB {
            base: db::JsonDBBase::new(),
            data: vec![],
        }
    }

    fn load(&mut self, contents: String) {
        let json: DB<Tag> = serde_json::from_str(&contents).unwrap();
        self.base = db::JsonDBBase {
            version: json.version,
            created_at: json.created_at,
            updated_at: json.updated_at,
        };
        self.data = json.data;
    }

    fn flush(&self) -> bool {
        let db_path = path::Path::new(".").join("db").join("tag.json");
        if db_path.exists() {
            let output: DB<Tag> = DB {
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

        for tag in self.data.iter_mut() {
            if tag.tag == key {
                let mut temp_posts: Vec<Self::Content> = vec![];
                for post in tag.posts.iter() {
                    if !post_names.contains(&post.name) {
                        temp_posts.push(post.clone().to_owned());
                    }
                }
                tag.posts = temp_posts;
            }
        }

        if should_flush {
            self.flush();
        }
    }

    fn remove_key(&mut self, key: String, should_flush: bool) {
        let mut tags_temp: Vec<Tag> = vec![];
        for tag in self.data.iter() {
            if tag.tag != key {
                tags_temp.push(tag.clone());
            }
        }

        self.data = tags_temp;
        if should_flush {
            self.flush();
        }
    }

    fn has_key(&self, key: String) -> bool {
        for tag in self.data.iter() {
            if tag.tag == key {
                return true;
            }
        }
        false
    }
}
