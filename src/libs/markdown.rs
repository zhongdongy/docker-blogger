use pulldown_cmark::{html, Options, Parser};
use std::error::Error;

pub fn markdown_to_html(content: &str) -> Result<String, Box<dyn Error>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content.trim(), options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::markdown_to_html;

    #[test]
    fn test_markdown_to_html() {
        let raw = "\n\n### 初心是什么？\n---\n你好";
        if let Ok(output) = markdown_to_html(raw) {
            assert_eq!(output, "<h3>初心是什么？</h3>\n<hr />\n<p>你好</p>");
        } else {
            panic!("Should be parsed without error!");
        }
    }
}
