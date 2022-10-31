from datetime import datetime

from flask import Flask, g
from flask_minify import Minify

from blueprints.post import bp_post
from libs.cache import build_page_cache
from libs.func import inline_css, inline_script
from libs.router import route_post
from utils.config import load_config


def create_app():
    app = Flask(__name__)
    # Minify(app=app, html=True, js=True, cssless=True, static=True)

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

    @app.route('/test')
    def test():
        build_page_cache(True)
        return "Hello World"

    app.register_blueprint(bp_post, url_prefix='/post')

    return app


if __name__ == '__main__':
    create_app().run()
