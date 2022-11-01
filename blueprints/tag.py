from os import getcwd
from os.path import exists, abspath, join, isfile

from flask import Blueprint, abort, send_file

bp_tag = Blueprint('tag', __name__)


@bp_tag.get("/<path:tag_name>/")
def view_tag(tag_name: str):
    tag_cache_path = abspath(join(getcwd(), f'cached/tag/{tag_name}.html'))
    if exists(tag_cache_path) and isfile(tag_cache_path):
        return send_file(tag_cache_path)
    return abort(404)
