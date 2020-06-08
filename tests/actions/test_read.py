import pathlib
import shutil
from typing import Union, Callable, Optional

import pytest

from engine.actions.read import inspect, ReadMode

TEST_DIR = pathlib.Path(__file__).parent.joinpath("test_assets")


@pytest.fixture(autouse=True)
def dir_test_fs():
    """ Create test directory for proper testing """
    if TEST_DIR.exists():
        shutil.rmtree(str(TEST_DIR))
    TEST_DIR.mkdir()
    yield
    shutil.rmtree(str(TEST_DIR))


@pytest.fixture()
def make_file(
        dir_test_fs
) -> Callable[[Union[bytes, str], Optional[str]], None]:
    """ Create test file for reading tests """
    def _content_pass(content: Union[bytes, str], mode: str = "w") -> None:
        with open(str(TEST_DIR / "test_file_read.txt"), mode) as writer:
            writer.write(content)
    return _content_pass


async def test_file_inspection(make_file):
    """ Checks: Basic test file inspection """
    content = """
    RAMADA
    RAMZAN
    xd
    bobby pin bobby pun
    """
    make_file(content)
    buffer = []
    async for chunk in inspect(TEST_DIR / "test_file_read.txt"):
        buffer.append(chunk)
    actual = "".join(buffer)
    assert actual == content


async def test_file_inspection_chunk_size(make_file):
    """ Checks: It is possible to regulate chunk size """
    content = "x" * 3
    make_file(content)
    buffer = []
    async for chunk in inspect(TEST_DIR / "test_file_read.txt", chunk_size=2):
        buffer.append(chunk)
    actual = len(buffer)
    assert actual == 2


async def test_file_inspection_bytes(make_file):
    """ Checks: It is possible to get bytes instead of string """
    content = b'0123456789'
    make_file(content, "wb")
    buffer = []
    async for chunk in inspect(
            TEST_DIR / "test_file_read.txt", mode=ReadMode.BYTES
    ):
        buffer.append(chunk)
    actual = buffer[0]
    assert actual == content
