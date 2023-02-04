import json
from os import getcwd
from os.path import abspath, join, exists

from flask import abort

from libs.cache import hit_page_file_cache


def route_post(post_name: str, *, is_index=False):
    content = None
    # Check home page first
    if is_index:
        content = hit_page_file_cache('__index__')

    # Check perm link indices first
    perm_link_index_path = abspath(join(getcwd(), 'cached/index/perm_link.json'))
    if exists(perm_link_index_path):
        with open(perm_link_index_path, 'r', encoding='utf-8') as perm_link_content:
            perm_link = json.loads(perm_link_content.read())
            if post_name in perm_link:
                content_name = perm_link[post_name]['name']
                content = hit_page_file_cache(content_name)
    if content is None:
        # Try with file cache
        content = hit_page_file_cache(post_name)

    if content is not None:
        return content
    return abort(404)
