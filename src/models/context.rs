use chrono::{ Utc};
use serde::{Deserialize, Serialize};

use crate::utils::config::Config;

#[derive(Serialize, Deserialize)]
pub struct ContextSiteInfo {
    pub beian_id: Option<String>,
    pub icp_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct Context {
    pub site_name: String,
    pub site_email: Option<String>,
    pub site_year: String,
    pub site_slogan: String,
    pub site_home: String,
    pub site_info: ContextSiteInfo,
    pub enable_beian: bool,
    pub baidu_site_verification: Option<String>,
}

impl Context {
    pub fn from_config(config: Config) -> Context {
        Context {
            site_name: config.site.site_name.clone(),
            site_email: config.site.site_email.clone(),
            site_year: Utc::now().format("%Y").to_string(),
            site_home: match config.site.enable_https {
                Some(enable) => match enable {
                    true => format!("https://{}", config.site.hostname).to_string(),
                    false => format!("http://{}", config.site.hostname).to_string(),
                },
                None => format!("http://{}", config.site.hostname).to_string(),
            },
            site_slogan: match config.site.site_slogan {
              Some(slogan) => slogan,
              None => "由 <a href=\"https://hub.docker.com/r/dongsxyz/rust_blogger\" target=\"_blank\">Eastwind Blogger</a> 驱动".to_string()
            } ,
            baidu_site_verification: config.site.baidu_site_verification.clone(),
            site_info: ContextSiteInfo { 
              beian_id: config.site.beian.beian_id.clone(),
               icp_id: config.site.beian.icp_id.clone(),
          },enable_beian: match config.site.beian.enabled {
            Some(b)=>b,
            None=>false,
          } 
        }
    }
}
