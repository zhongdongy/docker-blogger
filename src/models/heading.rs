use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Heading {
    pub id: String,
    pub tag: String,
    pub content: String,
}
impl Clone for Heading {
    fn clone(&self) -> Self {
        Heading {
            id: self.id.clone(),
            tag: self.tag.clone(),
            content: self.content.clone(),
        }
    }
}

impl Heading {
    pub fn new(text: String, level: u32) -> Self {
        let re = regex::Regex::new(r"[^\w\u4E00-\u9FFF]+").unwrap();

        let mut temp_id = re.replace_all(text.as_str(), "-").to_owned().to_string();
        temp_id = if let Some(t) = temp_id.strip_prefix("-") {
            t.to_string()
        } else {
            temp_id
        };
        temp_id = if let Some(t) = temp_id.strip_suffix("-") {
            t.to_string()
        } else {
            temp_id
        };

        Self {
            tag: format!("h{}", &level),
            id: temp_id,
            content: text,
        }
    }
}
