---
id: session-init
type: summary
status: unverified
title: Session::__init__
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.__init__
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.__init__@492d759b5f399e69
---
# Session::__init__

Takes a token, stores it on self.token, and sets self.issued to the current now() value.

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:5-7`
- **Signature:** `def __init__(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[store-now]]
