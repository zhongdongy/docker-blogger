import xml.etree.cElementTree as ET
from datetime import datetime

from models.index import TagIndexCollection, PostIndex
from utils.config import load_config
import urllib


def generate_sitemaps(tag_index: TagIndexCollection, *, archives=None):
    root_element = ET.Element('urlset')
    root_element.attrib['xmlns:xsi'] = "http://www.w3.org/2001/XMLSchema-instance"
    root_element.attrib[
        'xsi:schemaLocation'] = "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd"
    root_element.attrib['xmlns'] = "http://www.sitemaps.org/schemas/sitemap/0.9"
    root_element.attrib['xmlns:mobile'] = "http://www.baidu.com/schemas/sitemap-mobile/1/"

    config = load_config()
    url_root = ""
    if config.site.enable_https is True:
        url_root = f"https://{config.site.hostname}"
    else:
        url_root = f"http://{config.site.hostname}"

    posts: list[PostIndex] = list()

    for tag in tag_index.tags:
        _posts = tag_index.posts(tag)
        if len(_posts) == 0:
            continue
        posts.extend(_posts)
        # Include tag page
        tag_element = ET.SubElement(root_element, "url")
        ET.SubElement(tag_element, "loc").text = url_root + urllib.parse.quote(f'/tag/{tag}/')
        ET.SubElement(tag_element, "mobile:mobile", dict(type="pc,mobile"))
        ET.SubElement(tag_element, "lastmod").text = datetime.now().strftime("%Y-%m-%d")
        ET.SubElement(tag_element, "changefreq").text = "daily"
        ET.SubElement(tag_element, "priority").text = "0.8"

    if len(posts) > 0:
        deduplicated_posts: list[PostIndex] = list()
        post_names = set()

        # Deduplication
        for post in posts:
            if post.name not in post_names:
                deduplicated_posts.append(post)
                post_names.add(post.name)

        # Build post page
        posts = list(sorted(deduplicated_posts, key=lambda p: p.preamble.updated_at, reverse=True))
        for post in posts:
            post.name = post.name.replace('\\', '/')  # Unescape backslash
            if post.preamble.permanent_link is not None and len(post.preamble.permanent_link) > 0:
                post_element = ET.SubElement(root_element, "url")
                ET.SubElement(post_element, "loc").text = url_root + urllib.parse.quote(
                    f'/post/{post.preamble.permanent_link}/')
                ET.SubElement(post_element, "mobile:mobile", dict(type="pc,mobile"))
                ET.SubElement(post_element, "lastmod").text = post.preamble.updated_at.strftime("%Y-%m-%d")
                ET.SubElement(post_element, "changefreq").text = "weekly"
                ET.SubElement(post_element, "priority").text = "0.8"

            post_element = ET.SubElement(root_element, "url")
            ET.SubElement(post_element, "loc").text = url_root + urllib.parse.quote(f'/post/{post.name}/')
            ET.SubElement(post_element, "mobile:mobile", dict(type="pc,mobile"))
            ET.SubElement(post_element, "lastmod").text = post.preamble.updated_at.strftime("%Y-%m-%d")
            ET.SubElement(post_element, "changefreq").text = "weekly"
            ET.SubElement(post_element, "priority").text = "0.8"

    # Include tags page
    tags_element = ET.SubElement(root_element, "url")
    ET.SubElement(tags_element, "loc").text = url_root + f'/tags/'
    ET.SubElement(tags_element, "mobile:mobile", dict(type="pc,mobile"))
    ET.SubElement(tags_element, "lastmod").text = datetime.now().strftime("%Y-%m-%d")
    ET.SubElement(tags_element, "changefreq").text = "daily"
    ET.SubElement(tags_element, "priority").text = "0.8"

    # Include archives page
    archives_element = ET.SubElement(root_element, "url")
    ET.SubElement(archives_element, "loc").text = url_root + f'/archives/'
    ET.SubElement(archives_element, "mobile:mobile", dict(type="pc,mobile"))
    ET.SubElement(archives_element, "lastmod").text = datetime.now().strftime("%Y-%m-%d")
    ET.SubElement(archives_element, "changefreq").text = "daily"
    ET.SubElement(archives_element, "priority").text = "0.8"

    # Include archives page by year and month
    if archives is not None and len(archives) > 0:
        for year in archives:
            archives_element = ET.SubElement(root_element, "url")
            ET.SubElement(archives_element, "loc").text = url_root + f'/archives/{year}/'
            ET.SubElement(archives_element, "mobile:mobile", dict(type="pc,mobile"))
            ET.SubElement(archives_element, "lastmod").text = datetime.now().strftime("%Y-%m-%d")
            ET.SubElement(archives_element, "changefreq").text = "monthly"
            ET.SubElement(archives_element, "priority").text = "0.3"
            for month in archives[year]:
                archives_element = ET.SubElement(root_element, "url")
                ET.SubElement(archives_element, "loc").text = url_root + f'/archives/{year}/{month}/'
                ET.SubElement(archives_element, "mobile:mobile", dict(type="pc,mobile"))
                ET.SubElement(archives_element, "lastmod").text = datetime.now().strftime("%Y-%m-%d")
                ET.SubElement(archives_element, "changefreq").text = "monthly"
                ET.SubElement(archives_element, "priority").text = "0.3"

    tree = ET.ElementTree(root_element)
    tree.write('cached/sitemap.xml', encoding='utf-8', xml_declaration=True)
