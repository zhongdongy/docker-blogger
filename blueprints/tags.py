from os import getcwd
from os.path import exists, abspath, join, isfile

from flask import Blueprint, abort, send_file

bp_tags = Blueprint('tags', __name__)


@bp_tags.get("/")
def view_tags():
    tags_cache_path = abspath(join(getcwd(), f'cached/index/tags.html'))
    if exists(tags_cache_path) and isfile(tags_cache_path):
        return send_file(tags_cache_path)
    return abort(404)
