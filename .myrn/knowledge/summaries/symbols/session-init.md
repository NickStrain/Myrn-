---
id: session-init
type: summary
status: unverified
title: Session::__init__
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.__init__
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.__init__@492d759b5f399e69
map_version: '2'
---
# Session::__init__

Stores the given token and sets issued to the current now() value.

## Behaviour

- __init__("abc") -> self.token == "abc" and self.issued == now()
- No validation: any value, including None or an empty string, is stored as the token
- Reads the clock on every construction, so issued is set at creation, not at first use

## Terms

constructor, creation time, issue time, token storage, initialise

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:5-7`
- **Signature:** `def __init__(self, token)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[store-now]]
