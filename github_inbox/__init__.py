from flask import Flask

from . import routes


def create_app():
    app = Flask("github-inbox")
    app.add_url_rule("/", "index", routes.index)
    return app
