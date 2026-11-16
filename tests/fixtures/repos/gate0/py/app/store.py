class TokenStore:
    """Holds issued tokens."""

    def __init__(self):
        self.tokens = []

    def lookup(self, token):
        return token if token in self.tokens else None

    def insert(self, token):
        self.tokens.append(token)


def now():
    return 42
