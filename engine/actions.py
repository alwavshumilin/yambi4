import asyncio
import os
from pathlib import Path
from typing import Union

from aiofiles.os import wrap


os_walk = wrap(os.walk)


async def lurk(path: Union[Path, str]):
    """ """
    for root, directory, file in await os_walk(path):
        print(root, directory, file)


async def exec(coro):
    result = await asyncio.create_task(coro)
    return result
