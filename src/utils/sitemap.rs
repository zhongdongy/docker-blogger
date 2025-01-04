use crate::models::sitemap;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;
use quick_xml::ElementWriter;
use quick_xml::Error;
use std::io::Write;

pub fn generate_sitemap(locs: &Vec<sitemap::SitemapLoc>) -> Result<String, Error> {
    let mut buffer = Vec::new();

    let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

    writer
        .get_mut() // Updated due to <https://github.com/tafia/quick-xml/pull/568>
        .write(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
        .unwrap();

    let urlset = writer.create_element("urlset");
    urlset.with_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"))
    .with_attribute(("xsi:schemaLocation","http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd"))
    .with_attribute(("xmlns","http://www.sitemaps.org/schemas/sitemap/0.9"))
    .with_attribute(("xmlns:mobile","http://www.baidu.com/schemas/sitemap-mobile/1/"))
    .write_inner_content(|urlset: &mut Writer<&mut Vec<u8>>| ->Result<(), std::io::Error> {
        // Insert all loc nodes
        locs.iter().for_each(|loc_ele |{
            write_url_element(
                urlset.create_element("url"),
                &loc_ele.loc,
                &loc_ele.mobile_type,
                &loc_ele.lastmod,
                &loc_ele.changefreq,
                &loc_ele.priority
            );
        });
      Ok(())
    }).unwrap();

    Ok(std::str::from_utf8(&buffer).unwrap().to_string())
}

fn write_url_element<T>(
    url_writer: ElementWriter<T>,
    loc: &str,
    mobile_type: &str,
    lastmod: &str,
    changefreq: &str,
    priority: &str,
) where
    T: Write,
{
    url_writer
        .write_inner_content(
            |url| -> Result<(), std::io::Error> {
                url.create_element("loc")
                    .write_text_content(BytesText::new(&loc))
                    .unwrap();
                url.create_element("mobile:mobile")
                    .with_attribute(("type", mobile_type));
                url.create_element("lastmod")
                    .write_text_content(BytesText::new(lastmod))
                    .unwrap();
                url.create_element("changefreq")
                    .write_text_content(BytesText::new(changefreq))
                    .unwrap();
                url.create_element("priority")
                    .write_text_content(BytesText::new(&priority))
                    .unwrap();
                Ok(())
            },
        )
        .unwrap();
}
