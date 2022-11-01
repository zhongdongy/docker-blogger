from utils.config import load_config


def generate_robots_txt():
    # Generate robots.txt
    config = load_config()
    site_url = ""
    if config.site.enable_https:
        site_url = f"https://{config.site.hostname}"
    else:
        site_url = f"http://{config.site.hostname}"
    contents = list()
    contents.append(f"Sitemap: {site_url}/sitemap.xml")
    with open('robots.txt', 'w', encoding='utf-8') as robots_content:
        robots_content.write('\n'.join(contents))
