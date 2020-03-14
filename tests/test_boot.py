async def test_boot_app_web(app_client):
    """ Checks: App is ready to serve REST """
    resp = await app_client.get('/')
    assert resp.status == 200


async def test_boot_app_file_poller(app_client):
    """ Checks: App is ready to seek file changes """
    actual = app_client.app['event_handler']
    assert actual == 'printer'  # TODO: Fragile test detected
