from app.store import TokenStore
from app.session import refresh


def test_refresh_known_token():
    store = TokenStore()
    store.insert("abc")
    assert refresh(store, "abc") is not None
