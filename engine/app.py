from aiohttp import web

from engine.servlet import boot

if __name__ == '__main__':
    web.run_app(boot())
