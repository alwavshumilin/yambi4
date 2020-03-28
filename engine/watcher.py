import asyncio
import logging
import pathlib
from asyncio import AbstractEventLoop, Task
from functools import partial
from typing import Optional, Tuple, Awaitable, Callable, Union, cast

from aiohttp.abc import Application
from watchgod import awatch, DefaultDirWatcher, Change

logging.basicConfig(level=10)  # TODO: Aio logging


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


def setup(
        event_handler: EventHandler = printer,
        *,
        paths: Union[pathlib.Path, str],
        loop: Optional[AbstractEventLoop] = None,
) -> Callable[[Application], Awaitable[None]]:
    """ Setup file watcher for aio http web application """
    if loop is None:
        loop = asyncio.get_event_loop()

    async def run_watcher(application: Application) -> None:
        """ aio http app binder for file watcher """
        for path in paths:
            application['file_watcher'] = cast(
                AbstractEventLoop, loop
            ).create_task(observe(event_handler, path=path))

    return run_watcher


async def shutdown_watcher(application: Application) -> None:
    """ Graceful shutdown for file watcher w/ aiohttp server """
    watcher: Task = application['file_watcher']
    watcher.cancel()
