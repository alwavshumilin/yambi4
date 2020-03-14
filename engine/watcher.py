import asyncio
import logging
import pathlib
from asyncio import AbstractEventLoop
from functools import partial
from typing import Optional, Tuple, Awaitable, Callable, Union

from aiohttp.abc import Application
from watchgod import awatch, DefaultDirWatcher, Change

logging.basicConfig(level=10)


PATH = pathlib.Path(__file__).parent.parent.joinpath('test')
EventHandler = Callable[[Tuple[Change, str]], Awaitable[None]]


async def observe(
        event_handler: EventHandler,
        *,
        path: Union[pathlib.Path, str],
        loop: Optional[AbstractEventLoop] = None,
) -> None:
    """ File seeker based on watchgod asynchronous API """
    watcher = partial(awatch, watcher_cls=DefaultDirWatcher, loop=loop)
    async for changes in watcher(path):
        await event_handler(changes)


async def printer(changes: Tuple[Change, str]) -> None:
    """ Log and go out event handler """
    logging.info(changes)


def setup_for_app(
        event_handler: Optional[EventHandler] = printer,
        *,
        loop: Optional[AbstractEventLoop] = None,
) -> Callable[[Application], Awaitable[None]]:
    """ Setup file watcher for aio http web application """
    if loop is None:
        loop = asyncio.get_event_loop()

    async def run_with_app(application: Application) -> None:
        """ aio http app binder for file watcher """
        application['event_handler'] = event_handler.__qualname__
        loop.create_task(observe(event_handler, path=PATH))

    return run_with_app
