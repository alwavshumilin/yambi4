import asyncio
from pathlib import Path

from engine.actions import exec, lurk


if __name__ == '__main__':
    asyncio.run(exec(lurk(Path(__file__).parent)))
