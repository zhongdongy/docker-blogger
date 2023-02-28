use lazy_static::lazy_static;
use regex::Regex;

pub fn year_month_date(value: &str) -> (String, String, String) {
    lazy_static! {
        static ref DATE_MATCHER: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    }

    if let Some(captures) = DATE_MATCHER.captures(value) {
        if captures.len() == 4 {
            let year = captures.get(1).unwrap().as_str().to_string();
            let month = captures.get(2).unwrap().as_str().to_string();
            let date = captures.get(3).unwrap().as_str().to_string();
            return (year, month, date);
        }
    }

    (String::from("1970"), String::from("01"), String::from("01"))
}

#[cfg(test)]
mod tests {
    use super::year_month_date;

    #[test]
    fn test_date_extraction() {
        let (mut y, mut m, mut d) = year_month_date("2023-01-05");
        assert_eq!(&y, "2023");
        assert_eq!(&m, "01");
        assert_eq!(&d, "05");

        (y, m, d) = year_month_date("2023-1-05");
        assert_eq!(&y, "1970");
        assert_eq!(&m, "01");
        assert_eq!(&d, "01");
    }
}
