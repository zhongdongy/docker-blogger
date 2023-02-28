use serde::{Deserialize, Serialize};

use crate::utils::str::year_month_date;

use super::post::Post;

use super::date::Date;
use super::month::Month;

#[derive(Serialize, Deserialize)]
pub struct ArchiveByYear {
    pub year: String,
    pub months: Vec<ArchiveByMonth>,
}

#[derive(Serialize, Deserialize)]
pub struct ArchiveByMonth {
    pub month: Month,
    pub dates: Vec<ArchiveByDate>,
}

#[derive(Serialize, Deserialize)]
pub struct ArchiveByDate {
    pub date: Date,
    pub posts: Vec<Post>,
}

// region: Implement structs.

impl ArchiveByYear {
    pub fn posts(&self) -> Vec<Post> {
        let mut temp_posts = vec![];
        self.months.iter().for_each(|mon| {
            temp_posts.append(&mut mon.posts());
        });
        temp_posts
    }
    pub fn posts_by_month(&self, month: &str) -> Vec<Post> {
        let month = Month::from(month);
        let mut temp_posts = vec![];

        for mon in self.months.iter() {
            if mon.month == month {
                mon.dates
                    .iter()
                    .for_each(|date| temp_posts.append(&mut date.posts.clone().to_owned()));
            }
        }

        temp_posts
    }

    pub fn insert_post(&mut self, post: &mut Post) -> bool {
        let preamble = &post.preamble;
        let created_date_str = preamble.created_at.clone().unwrap();

        let (y, m, _) = year_month_date(&created_date_str);
        let mut inserted = false;

        if self.year == y {
            for month in self.months.iter_mut() {
                inserted = month.insert_post(post);
            }
            if !inserted {
                // Month doesn't exist yet, create one
                let mut new_month = ArchiveByMonth::new(&m, None);
                inserted = new_month.insert_post(post);
                self.months.push(new_month);
            }
        }
        inserted
    }
    pub fn new(year: &str, month: Option<&str>, date: Option<&str>) -> Self {
        let mut ret = Self {
            year: String::from(year),
            months: vec![],
        };
        if let Some(m) = month {
            ret.months.push(ArchiveByMonth::new(m, date));
        }
        ret
    }
}

impl ArchiveByMonth {
    pub fn posts(&self) -> Vec<Post> {
        let mut temp_posts = vec![];
        self.dates
            .iter()
            .for_each(|date| temp_posts.append(&mut date.posts.clone().to_owned()));
        temp_posts
    }

    pub fn posts_by_date(&self, date: Date) -> Vec<Post> {
        for d in self.dates.iter() {
            if d.date == date {
                return d.posts.clone().to_owned();
            }
        }
        vec![]
    }

    pub fn insert_post(&mut self, post: &mut Post) -> bool {
        let preamble = &post.preamble;
        let created_date_str = preamble.created_at.clone().unwrap();

        let (_, m, d) = year_month_date(&created_date_str);
        let mut inserted = false;

        let mon = Month::from(m.clone().as_str());
        if self.month == mon {
            for date in self.dates.iter_mut() {
                inserted = date.insert_post(post);
            }
            if !inserted {
                // Date doesn't exist yet, create one.
                let mut new_date = ArchiveByDate::new(&d);
                inserted = new_date.insert_post(post);
                self.dates.push(new_date);
            }
        }
        inserted
    }

    pub fn new(month: &str, date: Option<&str>) -> Self {
        let mut ret = Self {
            month: Month::from(month),
            dates: vec![],
        };

        if let Some(d) = date {
            ret.dates.push(ArchiveByDate::new(d));
        }

        ret
    }
}

impl ArchiveByDate {
    pub fn insert_post(&mut self, post: &mut Post) -> bool {
        let preamble = &post.preamble;
        let created_date_str = preamble.created_at.clone().unwrap();

        let (_, _, d) = year_month_date(&created_date_str);

        let d = Date::from(d.clone().as_str());

        if self.date == d {
            self.posts.push(post.to_owned());
            true
        } else {
            false
        }
    }
    pub fn new(date: &str) -> Self {
        Self {
            date: Date::from(date),
            posts: vec![],
        }
    }
}

// endregion

// region: Implement Clone trait for all structs.

impl Clone for ArchiveByYear {
    fn clone(&self) -> Self {
        Self {
            year: self.year.clone(),
            months: self.months.clone(),
        }
    }
}

impl Clone for ArchiveByMonth {
    fn clone(&self) -> Self {
        Self {
            month: self.month.clone(),
            dates: self.dates.clone(),
        }
    }
}

impl Clone for ArchiveByDate {
    fn clone(&self) -> Self {
        Self {
            date: self.date.clone(),
            posts: self.posts.clone(),
        }
    }
}
// endregion
