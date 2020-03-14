import pytest

from engine.servlet import boot


@pytest.fixture
async def app_client(aiohttp_client):
    """ Setup application for testing purposes """
    return await aiohttp_client(await boot())
