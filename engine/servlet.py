from aiohttp import web
from aiohttp.abc import Application

from .routes.client import root
from .watcher import setup_for_app


async def boot() -> Application:
    """ Application bootstrap entry point """
    app = web.Application()
    app.add_routes([
        web.get('/', root)
    ])
    watcher_fabric = setup_for_app()
    app.on_startup.append(watcher_fabric)
    return app
