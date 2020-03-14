import os
from pathlib import Path
from typing import Union

from aiofiles import open as aiopen
from aiofiles.os import wrap


os_walk = wrap(os.walk)

PathLike = Union[Path, str]


async def inspect(path: PathLike) -> str:
    """ TODO: """
    async with aiopen(path, encoding='utf-8') as reader:
        return await reader.read()
