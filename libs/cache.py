import json
import re
from datetime import datetime
from os import listdir, getcwd, mkdir
from os.path import exists, isfile, join, abspath, isdir, dirname

from flask import render_template, current_app
from markdown2 import Markdown

from .parser import parse_preamble, parse_headings
from .sitemap import generate_sitemaps
from models.index import PostIndex, PermanentLinkIndex, TagIndexCollection, PermanentLinkIndexCollection
from utils.config import load_config
from minify_html import minify

renderer = Markdown(extras=["footnotes", "fenced-code-blocks", "code-friendly", "header-ids", 'task_list', 'strikes'])


def _traverse_dir(abs_dirpath: str, ext='.md') -> list:
    files = list()
    if exists(abs_dirpath) and isdir(abs_dirpath):
        sub_paths = listdir(abs_dirpath)
        if len(sub_paths) > 0:
            for p in sub_paths:
                path = abspath(join(abs_dirpath, p))
                if isfile(path) and path.endswith(ext):
                    files.append(path)
                elif isdir(path):
                    files.extend(_traverse_dir(path, ext))
    return files


def hit_page_file_cache(name: str) -> str | None:
    cached_path = abspath(join(getcwd(), 'cached'))
    if not exists(cached_path):
        return None

    cached_file_path = abspath(join(cached_path, name + '.html'))
    if exists(cached_file_path) and isfile(cached_file_path):
        with open(cached_file_path, 'r', encoding='utf-8') as cached_file:
            return cached_file.read()
    return None


def load_blogs(dirname: str) -> list[dict]:
    blogs_path = abspath(join(getcwd(), dirname))
    blogs = _traverse_dir(blogs_path, '.md')
    blog_dicts = list()
    b: str
    regex = re.compile(r'(\.md)$')
    for b in blogs:
        save_path = b.replace(blogs_path, '').strip('/').strip('\\')
        save_path = regex.sub('.html', save_path)
        save_path = abspath(join(getcwd(), 'cached', save_path))
        blog_dicts.append(dict(
            path=b,
            save_path=save_path
        ))
    return blog_dicts


def save_cache_file(save_path, contents: str):
    contents = minify(
        contents,
        do_not_minify_doctype=True,
        keep_comments=False,
        keep_closing_tags=True,
        keep_html_and_head_opening_tags=True,
        minify_css=True,
        minify_js=True
    )
    dir_name = dirname(save_path)
    if not exists(dir_name):
        mkdir(dir_name)
    with open(save_path, 'w', encoding='utf-8') as output:
        output.write(contents)


def build_page_cache(clear_cached=False):
    blogs_path = abspath(join(getcwd(), 'blogs'))
    blogs = load_blogs('blogs')

    config = load_config()
    if config.public.home_post is None:
        config.public.home_post = 'index'
    home_page = config.public.home_post
    # Generate home_post markdown file if not exists
    home_page_path = join(blogs_path, home_page + '.md')
    if not exists(home_page_path):
        with open(home_page_path, 'w', encoding='utf-8') as home_page_content:
            home_page_content.write('Eastwind Simple Blogging\nWelcome!')
    blogs.append(dict(
        path=home_page_path,
        save_path=abspath(join(getcwd(), 'cached', home_page + '.html')),
        home_page=True
    ))

    if clear_cached:
        import shutil
        cached_path = abspath(join(getcwd(), 'cached'))
        if exists(cached_path):
            shutil.rmtree(cached_path)
        mkdir(cached_path)

    if len(blogs) > 0:
        tags = TagIndexCollection()
        perm_links = PermanentLinkIndexCollection()

        with current_app.app_context():
            for blog in blogs:
                with open(blog['path'], 'r', encoding='utf-8') as blog_content:
                    markdown_raw = blog_content.read()
                    preamble, content = parse_preamble(markdown_raw)
                    context = dict(
                        html=renderer.convert(content),
                        tags=list(),
                        site_tags=tags.tags
                    )
                    if preamble is not None:
                        context = {**context, **dict(
                            tags=preamble.tags,
                            title=preamble.title,
                            author=preamble.author,
                            created_at=preamble.created_at.timestamp(),
                            updated_at=preamble.updated_at.timestamp()
                        )}
                        if 'content-serif' in preamble.renderer_params:
                            context['flag_content_serif'] = 1
                        if 'disable-toc' in preamble.renderer_params:
                            context['flag_disable_toc'] = 1
                    else:
                        context = {**context, **dict(
                            tags=[],
                            title="",
                            author="",
                            created_at=datetime.now().timestamp(),
                            updated_at=datetime.now().timestamp()
                        )}
                    if 'home_page' in blog and blog['home_page'] is True:
                        html = render_template('home_page.jinja2', **context)
                    else:
                        html = render_template('blog_page.jinja2', **context)

                    # Get headings
                    headings = parse_headings(html)
                    for heading in headings:
                        if heading['id'].startswith('-') or re.match(r'^\d.*', heading['id']):
                            # Should add extra characters
                            new_id = 'padding' + heading['id']
                            html = html.replace(f'id="{heading["id"]}"', f'id="{new_id}"')
                            heading['id'] = new_id

                    headings_json = json.dumps(headings, ensure_ascii=False, indent=None)
                    html = html.replace('@{HEADINGS_JSON}', headings_json)

                    if preamble is not None:
                        tag_names = preamble.tags
                        perm_link = preamble.permanent_link
                        name: str = blog['path'].replace(blogs_path, '').strip('/').strip('\\')
                        if name.endswith('.md'):
                            name = name[:-3]
                        if len(tag_names) > 0:
                            # Include tags
                            for tag_name in tag_names:
                                tag = PostIndex(dict(
                                    tag=tag_name,
                                    name=name,
                                    preamble=preamble
                                ))
                                tags.add(tag)
                        else:
                            # No provided tags, set to Others
                            tags.add(PostIndex(dict(
                                tag="其他",
                                name=name,
                                preamble=preamble
                            )))
                        if perm_link is not None:
                            perm_links.add(PermanentLinkIndex(dict(
                                permanent_link=perm_link,
                                name=name,
                                preamble=preamble
                            )))

                    save_cache_file(blog['save_path'], html)
        # Write cached indices to cached directory
        cached_index_path = abspath(join(getcwd(), 'cached/index'))
        if not exists(cached_index_path):
            mkdir(cached_index_path)

        if tags.length > 0:
            with open(join(cached_index_path, 'tag.json'), 'w', encoding='utf-8') as tag_index_content:
                tag_index_content.write(json.dumps(tags.to_dict(), ensure_ascii=False, indent=2))
            generate_sitemaps(tags)

            # Build tag page cache
            build_tag_page_cache(tags)

            # Build tags page cache
            build_tags_page_cache(tags)
        if len(perm_links) > 0:
            with open(join(cached_index_path, 'perm_link.json'), 'w', encoding='utf-8') as perm_link_index_content:
                perm_link_index_content.write(json.dumps(perm_links.to_dict(), ensure_ascii=False, indent=2))


def build_tag_page_cache(tag_index: TagIndexCollection):
    cached_path = abspath(join(getcwd(), 'cached/tag'))
    for tag_name in tag_index.tags:
        posts = tag_index.posts(tag_name)
        if len(posts) > 0:
            posts = list(sorted(posts, key=lambda p: p.preamble.updated_at, reverse=True))
            html = render_template("tag.jinja2", **dict(posts=posts, tag_name=tag_name))
            save_path = join(cached_path, f'{tag_name}.html')
            save_cache_file(save_path, html)


def build_tags_page_cache(tag_index: TagIndexCollection):
    cached_path = abspath(join(getcwd(), 'cached/index'))
    tags = tag_index.tags
    html = render_template("tags.jinja2", **dict(tags=tags))
    save_path = join(cached_path, f'tags.html')
    save_cache_file(save_path, html)


if __name__ == '__main__':
    build_page_cache()
