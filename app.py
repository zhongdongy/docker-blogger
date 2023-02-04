from datetime import datetime
from os import getcwd
from os.path import exists, abspath, join, isfile

from flask import Flask, g, send_file, abort, render_template

from blueprints.archive import bp_archive
from blueprints.post import bp_post
from blueprints.tag import bp_tag
from blueprints.tags import bp_tags
from libs.cache import build_page_cache
from libs.func import inline_css, inline_script
from libs.robots import generate_robots_txt
from libs.router import route_post
from utils.config import load_config


def create_app():
    app = Flask(__name__)

    @app.context_processor
    def context_utils():

        config = load_config()

        g.site_name = config.site.site_name
        g.site_email = config.site.site_email
        g.site_year = datetime.now().year
        g.site_slogan = config.site.site_slogan
        if config.site.enable_https:
            g.site_home = f"https://{config.site.hostname}/"
        else:
            g.site_home = f"http://{config.site.hostname}/"

        g.site_info = dict(
            beian_id=config.site.beian.beian_id,
            icp_id=config.site.beian.icp_id
        )
        if config.site.beian.enabled is True:
            g.enable_beian = True

        if config.site.baidu_site_verification is not None:
            g.baidu_site_verification = config.site.baidu_site_verification

        return dict(
            inline_css=inline_css,
            inline_js=inline_script,
            len=len
        )

    @app.route('/')
    def home_page():  # put application's code here
        config = load_config()
        return route_post(config.public.home_post)

    @app.get('/baidu_verify_<string:some_path>.html')
    def baidu_site_verify(some_path: str):
        filepath = abspath(join(getcwd(), f"baidu_verify_{some_path}.html"))
        if exists(filepath) and isfile(filepath):
            return send_file(filepath)
        return abort(404)

    @app.route('/privacy-policy/')
    def privacy_policy_page():
        tags_cache_path = abspath(join(getcwd(), f'cached/site/privacy-policy.html'))
        if exists(tags_cache_path) and isfile(tags_cache_path):
            return send_file(tags_cache_path)
        return abort(404)

    generate_robots_txt()

    @app.route('/robots.txt')
    def serve_robots():
        return send_file('robots.txt')

    @app.route('/favicon.ico')
    def serve_favicon():
        if exists('favicon.ico'):
            return send_file('favicon.ico')
        return abort(404)

    @app.route('/sitemap.xml')
    def serve_sitemap():
        return send_file('cached/sitemap.xml')

    @app.route('/admin/cache-reload/')
    def reload_cache():
        build_page_cache(True)
        return "Completed"

    app.register_blueprint(bp_post, url_prefix='/post')
    # app.register_blueprint(bp_api, url_prefix='/api')
    app.register_blueprint(bp_tag, url_prefix='/tag')
    app.register_blueprint(bp_tags, url_prefix='/tags')
    app.register_blueprint(bp_archive, url_prefix='/archives')

    @app.errorhandler(404)
    def handle_404_page(error):
        return render_template('404.jinja2'), 404

    return app


if __name__ == '__main__':
    create_app().run()
