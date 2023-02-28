use crate::models::db::DB;
use crate::models::post::Post;
use crate::utils::db;
use crate::{models::archive::*, utils::str::year_month_date};
use std::{fs, path};

pub struct ArchiveDB {
    base: db::JsonDBBase,
    data: Vec<ArchiveByYear>,
}
impl ArchiveDB {
    pub fn insert_posts(&mut self, posts: &mut Vec<Post>, should_flush: bool) {
        // Iterate and find year
        posts.sort_by(|p1, p2| p1.preamble.created_at.cmp(&p2.preamble.created_at));
        posts.iter_mut().for_each(|p| {
            let mut inserted = false;
            let (y, m, d) = year_month_date(&p.preamble.created_at.clone().unwrap());

            self.data.iter_mut().for_each(|a| {
                inserted = a.insert_post(p);
            });

            // if m == String::from("01") && d == String::from("01") {
            //     println!("post: {}", &p.name);
            // }

            if !inserted {
                let mut year = ArchiveByYear::new(&y, Some(&m), Some(&d));
                year.insert_post(p);
                self.data.push(year);
            }
        });

        if should_flush {
            db::JsonDatabase::flush(self);
        }
    }

    pub fn by_year(&self, year: String) -> Option<ArchiveByYear> {
        for y in self.data.iter() {
            if y.year == year {
                return Some(y.clone());
            }
        }
        None
    }

    pub fn archives(&self) -> Vec<ArchiveByYear> {
        self.data.clone()
    }
}

impl db::JsonDatabase for ArchiveDB {
    type Item = ArchiveByYear;
    type Content = Post;

    fn query_posts(&self, year: String) -> Option<Vec<Self::Content>> {
        let mut res: Vec<Post> = vec![];

        self.data.iter().for_each(|t| {
            if t.year == year {
                res.append(&mut t.posts().clone().to_owned());
            }
        });

        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }

    fn insert_post(&mut self, _year: String, post: &mut Self::Content, should_flush: bool) {
        self.insert_posts(&mut vec![post.to_owned()], false);

        if should_flush {
            self.flush();
        }
    }

    fn count_posts(&self, key: Option<String>) -> usize {
        match key {
            None => self.data.iter().map(|y| y.posts().len()).sum(),
            Some(k) => {
                let mut count = 0;
                self.data.iter().for_each(|t| {
                    if t.year == k {
                        count += t.posts().len();
                    }
                });
                count
            }
        }
    }

    fn new() -> Self {
        ArchiveDB {
            base: db::JsonDBBase::new(),
            data: vec![],
        }
    }

    fn load(&mut self, contents: String) {
        let json: DB<ArchiveByYear> = serde_json::from_str(&contents).unwrap();
        self.base = db::JsonDBBase {
            version: json.version,
            created_at: json.created_at,
            updated_at: json.updated_at,
        };
        self.data = json.data;
    }

    fn flush(&self) -> bool {
        let db_path = path::Path::new(".").join("db").join("archive.json");
        if db_path.exists() {
            let output: DB<ArchiveByYear> = DB {
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

    fn remove_posts(&mut self, _key: String, _posts: &mut Vec<Self::Content>, _should_flush: bool) {
        todo!()
    }

    fn remove_key(&mut self, _key: String, _should_flush: bool) {
        panic!("Unsupported function call")
    }

    fn has_key(&self, year: String) -> bool {
        for au in self.data.iter() {
            if au.year == year {
                return true;
            }
        }
        false
    }
}
