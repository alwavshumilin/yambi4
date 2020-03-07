import os
from pathlib import Path
from typing import Union, Tuple, Iterator

from aiofiles import open as aiopen
from aiofiles.os import wrap

PathEd = Union[Path, str]

os_walk = wrap(os.walk)


async def lurk(path: PathEd) -> Iterator[Tuple[Path, str, str]]:
    """ TODO: """
    for root, directory, file in await os_walk(path):
        yield Path(root), directory, file


async def inspect(path: PathEd) -> str:
    """ TODO: """
    async with aiopen(path, encoding='utf-8') as reader:
        return await reader.read()
