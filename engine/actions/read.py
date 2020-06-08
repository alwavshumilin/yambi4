from asyncio import AbstractEventLoop
from enum import Enum
from pathlib import Path
from typing import Union, Generator

from aiofiles import open as aiopen

PathLike = Union[Path, str]
FileContent = Union[bytes, str]


class ReadMode(Enum):
    """ Non-alternative reading modes """
    BYTES = 'rb'
    STRING = 'r'


async def inspect(
        path: PathLike,
        loop: AbstractEventLoop,
        mode: ReadMode = ReadMode.STRING,
        chunk_size: int = 1024,
) -> Generator[None, FileContent, None]:
    """ Get the file content in lazy fashion """
    async with aiopen(path, mode=mode.value, loop=loop) as reader:
        while True:
            content = await reader.read(chunk_size)
            if not content:
                break
            yield content
    return


def tail():
    """ TODO: tailf or asyncio.subprocess w/ real tail? """
