from aiohttp import web


class EventHandler(web.View):
    """ TODO: """

    async def get(self):
        """ TODO: """
        ws = web.WebSocketResponse()
        return ws
