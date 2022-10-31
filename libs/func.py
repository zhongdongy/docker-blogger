from os import getcwd
from os.path import exists, isfile, join, abspath, isdir


def _get_path(name='css'):
    static_path = abspath(join(getcwd(), 'static'))
    return join(static_path, name)


def inline_css(name: str):
    css_path = _get_path('css')
    if not name.endswith('.css'):
        name += '.css'
    css_file_path = join(css_path, name)
    if not exists(css_path) or not isdir(css_path) or not exists(css_file_path) or not isfile(css_file_path):
        return ""

    with open(css_file_path, 'r', encoding='utf-8') as css_file:
        return f"<style>{css_file.read()}</style>"


def inline_script(name: str):
    js_path = _get_path('script')
    if not name.endswith('.js'):
        name += '.js'
    js_file_path = join(js_path, name)
    if not exists(js_path) or not isdir(js_path) or not exists(js_file_path) or not isfile(js_file_path):
        return ""

    with open(js_file_path, 'r', encoding='utf-8') as js_file:
        return f"<script>{js_file.read()}</script>"
