use super::config::get_config;
use md5;

pub fn get_gravatar_url(email: &str) -> String {
    let digest = md5::compute(email.as_bytes());
    let gravatar_url = "https://www.gravatar.com/avatar/";
    let config = get_config(None);

    if let Some(gravatar_proxy) = config.site.gravatar_proxy {
        let mut url = gravatar_proxy.clone();
        if !gravatar_url.ends_with('/') {
            url.push('/');
        }
        return format!("{}{:x}", &url, digest);
    }

    format!("{}{:x}", gravatar_url, digest)
}
