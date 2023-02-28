use crate::models::db::DB;
use chrono::Utc;
use serde;
use std::fs;
use std::path;

pub trait JsonDatabase {
    type Item;
    type Content;

    fn query_posts(&self, key: String) -> Option<Vec<Self::Content>>;
    fn load(&mut self, contents: String);
    fn insert_post(&mut self, key: String, post: &mut Self::Content, should_flush: bool);
    fn remove_posts(&mut self, key: String, posts: &mut Vec<Self::Content>, should_flush: bool);
    fn remove_key(&mut self, key: String, should_flush: bool);
    fn count_posts(&self, key: Option<String>) -> usize;
    fn has_key(&self, key: String) -> bool;
    fn new() -> Self;
    fn flush(&self) -> bool;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JsonDBBase {
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
}

impl JsonDBBase {
    pub fn new() -> Self {
        let version: &str = env!("CARGO_PKG_VERSION");
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            version: String::from(version),
            created_at: now.to_owned(),
            updated_at: now,
        }
    }
}

pub enum DatabaseSource {
    Tag,
    Archive,
    Author,
    Permlink,
}

pub fn get_database<T, V>(ty: DatabaseSource) -> T
where
    T: JsonDatabase<Item = V>,
    V: serde::Serialize,
{
    let db_name = match ty {
        DatabaseSource::Archive => "archive.json",
        DatabaseSource::Author => "author.json",
        DatabaseSource::Tag => "tag.json",
        DatabaseSource::Permlink => "permlink.json",
    };

    let db_path = path::Path::new(".").join("db").join(db_name);
    if !db_path.exists() {
        // Create database file if not exists.
        fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        let init_db: DB<i32> = DB::new();
        let init_db_data = serde_json::to_string(&init_db).unwrap();
        fs::write(db_path.to_owned(), init_db_data).unwrap();
    }

    let db_contents = fs::read_to_string(db_path).unwrap();
    let mut db: T = T::new();
    db.load(db_contents);

    db
}

pub fn clear_database() {
    let db_path = path::Path::new(".").join("db");
    if db_path.exists() {
        fs::remove_dir_all(db_path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::{clear_database, get_database, DatabaseSource, JsonDatabase};
    use crate::{libs::db::tag::TagDB, models::tag::Tag};

    #[test]
    fn test_get_database() {
        clear_database();
        sleep(Duration::from_secs(1));
        let db: TagDB = get_database::<TagDB, Tag>(DatabaseSource::Tag);
        assert_eq!(db.count_posts(None), 0)
    }

    #[test]
    fn test_database_insert() {
        clear_database();
        sleep(Duration::from_secs(3));
        let mut db: TagDB = get_database::<TagDB, Tag>(DatabaseSource::Tag);
        let mut tag = Tag {
            tag: String::from("hello"),
            posts: vec![],
        };

        db.insert_posts(String::from("hello"), &mut tag.posts, false);
        assert_eq!(db.flush(), true);
        assert_eq!(db.count_posts(None), 1);
        db.insert_posts(String::from("hello1"), &mut tag.posts, true);
        assert_eq!(db.count_posts(None), 2);
    }

    #[test]
    fn test_database_query() {
        clear_database();
        sleep(Duration::from_secs(2));
        let mut db: TagDB = get_database::<TagDB, Tag>(DatabaseSource::Tag);
        let mut tag = Tag {
            tag: String::from("hello"),
            posts: vec![],
        };

        db.insert_posts(String::from("hello"), &mut tag.posts, true);
        let mut val = db.query_posts(String::from("hello")).unwrap_or(vec![]);
        assert_eq!(val.len(), 1);
        val = db.query_posts(String::from("hello1")).unwrap_or(vec![]);
        assert_eq!(val.len(), 0);
    }
}
