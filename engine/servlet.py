import pathlib
from typing import Mapping, Any

import toml
from aiohttp import web
from aiohttp.abc import Application

from .routes.client import root
from .watcher import setup, shutdown_watcher


CFG_PATH = (
    pathlib.Path(__file__)
    .parent
    .joinpath('configuration')
    .joinpath('config.toml')
)

STATIC_PATH = pathlib.Path(__file__).parent.parent.joinpath('static')


def read_config() -> Mapping[str, Any]:
    """ Read config from fs """
    with open(CFG_PATH) as reader:
        return toml.decoder.load(reader)


async def boot() -> Application:
    """ Application bootstrap entry point """
    app = web.Application()
    app.add_routes([
        web.get('/', root),
        web.static('/static', STATIC_PATH, follow_symlinks=True)
    ])
    config = read_config()
    watcher_fabric = setup(paths=config['paths'], loop=app._loop)
    app.on_startup.append(watcher_fabric)
    app.on_cleanup.append(shutdown_watcher)
    return app
