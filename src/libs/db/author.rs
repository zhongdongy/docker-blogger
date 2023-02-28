use crate::models::author::Author;
use crate::models::db::DB;
use crate::models::post::Post;
use crate::utils::db;
use std::{fs, path};

pub struct AuthorDB {
    base: db::JsonDBBase,
    data: Vec<Author>,
}
impl AuthorDB {
    pub fn insert_posts(&mut self, key: String, posts: &mut Vec<Post>, should_flush: bool) {
        let mut inserted = false;
        self.data.iter_mut().for_each(|t| {
            if t.author == key {
                t.posts.append(&mut posts.to_owned());
                inserted = true;
            }
        });
        if !inserted {
            self.data.push(Author {
                author: key,
                posts: posts.to_owned(),
            })
        }

        if should_flush {
            db::JsonDatabase::flush(self);
        }
    }
}

impl db::JsonDatabase for AuthorDB {
    type Item = Author;
    type Content = Post;

    fn query_posts(&self, key: String) -> Option<Vec<Self::Content>> {
        let mut res: Vec<Post> = vec![];

        self.data.iter().for_each(|t| {
            if t.author == key {
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
            if t.author == key {
                t.posts.push(post.to_owned());
                inserted = true;
            }
        });
        if !inserted {
            self.data.push(Self::Item {
                author: key,
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
                    if t.author == k {
                        count += 1;
                    }
                });
                count
            }
        }
    }

    fn new() -> Self {
        AuthorDB {
            base: db::JsonDBBase::new(),
            data: vec![],
        }
    }

    fn load(&mut self, contents: String) {
        let json: DB<Author> = serde_json::from_str(&contents).unwrap();
        self.base = db::JsonDBBase {
            version: json.version,
            created_at: json.created_at,
            updated_at: json.updated_at,
        };
        self.data = json.data;
    }

    fn flush(&self) -> bool {
        let db_path = path::Path::new(".").join("db").join("author.json");
        if db_path.exists() {
            let output: DB<Author> = DB {
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

        for au in self.data.iter_mut() {
            if au.author == key {
                let mut temp_posts: Vec<Self::Content> = vec![];
                for post in au.posts.iter() {
                    if !post_names.contains(&post.name) {
                        temp_posts.push(post.clone().to_owned());
                    }
                }
                au.posts = temp_posts;
            }
        }

        if should_flush {
            self.flush();
        }
    }

    fn remove_key(&mut self, key: String, should_flush: bool) {
        let mut authors_temp: Vec<Author> = vec![];
        for au in self.data.iter() {
            if au.author != key {
                authors_temp.push(au.clone());
            }
        }

        self.data = authors_temp;
        if should_flush {
            self.flush();
        }
    }

    fn has_key(&self, key: String) -> bool {
        for au in self.data.iter() {
            if au.author == key {
                return true;
            }
        }
        false
    }
}
