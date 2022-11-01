from flask import Blueprint, abort, request

from libs.router import route_post

bp_api = Blueprint('api', __name__)


@bp_api.post("/analysis/")
def analysis():
    if request.headers.get('Content-Type') != 'application/json':
        return abort(405)

    remote_ip = request.remote_addr
    current_page = request.json.get('current_page')
    user_agent = request.user_agent

    return "Hello World"

# @bp_api.post("/feedback/")