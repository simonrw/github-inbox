import os
from dataclasses import dataclass, asdict
from concurrent.futures import ThreadPoolExecutor, wait

import requests
from flask import Flask

GITHUB_TOKEN = os.environ["GITHUB_TOKEN"]

# TODO: pagination

class GitHub:
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({
            "Authorization": f"Bearer {GITHUB_TOKEN}",
            })

    def assigned_issues(self):
        url = f"issues"
        return self._request(url)


    def _request(self, url, **query_params):
        full_url = f"https://api.github.com/{url}"
        r = self.session.get(full_url, params=query_params)
        r.raise_for_status()
        return r.json()


@dataclass
class Response:
    assigned_issues: list

    def to_dict(self) -> dict:
        return asdict(self)


class App:
    def __init__(self, token):
        self.token = token
        self.pool = ThreadPoolExecutor()
        self.github = GitHub()

    # routes
    def index(self):
        return "Hello world"


    # API methods
    def assigned_issues(self) -> dict:
        return self.github.assigned_issues()


def create_app():
    internal_app = App(GITHUB_TOKEN)

    app = Flask("github-inbox")
    app.add_url_rule("/", "index", internal_app.index)
    app.add_url_rule("/fetch/assigned-issues", "assigned-issues", internal_app.assigned_issues)
    return app
