import json
from os import getcwd
from os.path import exists, abspath, join, isfile

from flask import Blueprint, abort, send_file, render_template

bp_archive = Blueprint('archives', __name__)


@bp_archive.get("/")
def view_all():
    date_index_path = abspath(join(getcwd(), f'cached/archives/archives.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)

    return abort(404)


@bp_archive.get("/<int:year>/")
def view_year(year: int):
    date_index_path = abspath(join(getcwd(), f'cached/archives/{year:>04}.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)
    return abort(404)


@bp_archive.get("/<int:year>/<int:month>/")
def view_month(year: int, month: int):
    date_index_path = abspath(join(getcwd(), f'cached/archives/{year:>04}-{month:>02}.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)
    return abort(404)
