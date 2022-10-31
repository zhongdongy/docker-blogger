from flask import Blueprint, abort

from libs.router import route_post

bp_post = Blueprint('post', __name__)


@bp_post.get("/<path:post_name>/")
def view_post(post_name: str):
    return route_post(post_name)
