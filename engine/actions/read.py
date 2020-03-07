import os
from pathlib import Path
from typing import Union, Tuple, Iterator

from aiofiles.os import wrap


os_walk = wrap(os.walk)


async def lurk(path: Union[Path, str]) -> Iterator[Tuple[Path, str, str]]:
    """ TODO: """
    for root, directory, file in await os_walk(path):
        yield Path(root), directory, file
