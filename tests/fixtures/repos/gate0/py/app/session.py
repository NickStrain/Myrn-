from .store import TokenStore, now


class Session:
    def __init__(self, token):
        self.token = token
        self.issued = now()

    def is_fresh(self):
        return self.issued + 10 > now()


def refresh(store: TokenStore, token):
    """Issues a new session when the token is known."""
    found = store.lookup(token)
    if found is None:
        return None
    s = Session(found)
    return s if s.is_fresh() else None
