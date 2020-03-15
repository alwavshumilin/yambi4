from aiohttp import web
from aiohttp.abc import Application

from .routes.client import root
from .watcher import setup, shutdown_watcher


async def boot() -> Application:
    """ Application bootstrap entry point """
    app = web.Application()
    app.add_routes([
        web.get('/', root)
    ])
    watcher_fabric = setup()
    app.on_startup.append(watcher_fabric)
    app.on_cleanup.append(shutdown_watcher)
    return app
