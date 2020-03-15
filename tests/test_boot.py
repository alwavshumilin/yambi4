from asyncio import Task


async def test_boot_app_web(app_client):
    """ Checks: App is ready to serve REST """
    resp = await app_client.get('/')
    assert resp.status == 200


async def test_boot_app_file_poller(app_client):
    """ Checks: App is ready to seek file changes """
    file_watcher = app_client.app['file_watcher']
    assert isinstance(file_watcher, Task) is True
