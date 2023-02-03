import json
from os import getcwd
from os.path import exists, abspath, join, isfile

from flask import Blueprint, abort, send_file, render_template

bp_dates = Blueprint('dates', __name__)


@bp_dates.get("/")
def view_all():
    date_index_path = abspath(join(getcwd(), f'cached/dates/dates.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)
        # with open(date_index_path, 'r', encoding='utf-8') as dates_index:
        #     dates = json.load(dates_index)
        #     return render_template("dates.jinja2", dates= dates)

    return abort(404)


@bp_dates.get("/<int:year>/")
def view_year(year: int):
    date_index_path = abspath(join(getcwd(), f'cached/dates/{year:>04}.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)
    return abort(404)


@bp_dates.get("/<int:year>/<int:month>/")
def view_month(year: int, month: int):
    date_index_path = abspath(join(getcwd(), f'cached/dates/{year:>04}-{month:>02}.html'))
    if exists(date_index_path) and isfile(date_index_path):
        return send_file(date_index_path)
    return abort(404)
