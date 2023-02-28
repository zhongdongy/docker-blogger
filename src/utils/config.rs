use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicConfig {
    pub home_post: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeianConfig {
    pub enabled: Option<bool>,
    pub icp_id: Option<String>,
    pub beian_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    pub hostname: String,
    pub enable_https: Option<bool>,
    pub site_name: String,
    pub site_email: Option<String>,
    pub site_slogan: Option<String>,
    pub gravatar_proxy: Option<String>,
    pub baidu_site_verification: Option<String>,
    pub beian: BeianConfig,
    pub timezone: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub public: PublicConfig,
    pub site: SiteConfig,
}

///
pub fn get_config(config_file_name: Option<String>) -> Config {
    let config_file = config_file_name.unwrap_or(String::from("config.yml"));
    let config_file_path = Path::new(".").join(config_file);
    if config_file_path.is_file() && config_file_path.exists() {
        let config_contents =
            fs::read_to_string(config_file_path).expect("Should be able to read the config file");

        serde_yaml::from_str(&config_contents).expect("Config file is not properly formatted")
    } else {
        panic!(
            "Given config file path {} doesn't exists!",
            config_file_path.to_str().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_config() {
        use super::get_config;

        let config = get_config(Some(String::from("config.yml")));

        assert_eq!(config.site.hostname, "dongs.xyz");
    }
}
