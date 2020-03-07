import pytest

from engine.servlet import boot


@pytest.fixture
async def app_client(aiohttp_server, aiohttp_client):
    return await aiohttp_client(await boot())


async def test_boot_app(app_client):
    """ Checks: App is ready to be loaded """
    resp = await app_client.get('/')
    assert resp is not None
