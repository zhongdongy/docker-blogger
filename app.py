from datetime import datetime
from os.path import exists

from flask import Flask, g, send_file, abort

from blueprints.api import bp_api
from blueprints.post import bp_post
from blueprints.tag import bp_tag
from libs.cache import build_page_cache
from libs.func import inline_css, inline_script
from libs.robots import generate_robots_txt
from libs.router import route_post
from utils.config import load_config


def create_app():
    app = Flask(__name__)

    @app.context_processor
    def context_utils():
        g.site_name = "那阵东风"
        g.site_year = datetime.now().year
        g.site_slogan = "由 The Eastwind Blogger 驱动"

        config = load_config()

        g.site_info = dict(
            beian_id=config.site.beian.beian_id,
            icp_id=config.site.beian.icp_id
        )
        if config.site.beian.enabled is True:
            g.enable_beian = True

        return dict(
            inline_css=inline_css,
            inline_js=inline_script
        )

    @app.route('/')
    def home_page():  # put application's code here
        config = load_config()
        return route_post(config.public.home_post)

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

    return app


if __name__ == '__main__':
    create_app().run()
