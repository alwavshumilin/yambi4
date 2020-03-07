from aiohttp import web
from aiohttp.abc import Application


async def boot() -> Application:
    """ TODO: """
    return web.Application()
