pub struct SitemapLoc {
    pub loc: String,
    pub changefreq: String,
    pub lastmod: String,
    pub priority: String,
    pub mobile_type: String,
}

impl SitemapLoc {
    pub fn new(loc: String, lastmod: String) -> Self {
        SitemapLoc {
            loc,
            changefreq: String::from("daily"),
            lastmod,
            priority: String::from("0.8"),
            mobile_type: String::from("pc,mobile"),
        }
    }
}
