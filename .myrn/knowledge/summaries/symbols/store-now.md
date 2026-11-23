---
id: store-now
type: summary
status: unverified
title: store::now
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.now
anchors:
- sym:tests/fixtures/repos/gate0/py/app/store.py#tests.fixtures.repos.gate0.py.app.store.now@efdc8b33d4e62beb
map_version: '2'
---
# store::now

Returns the constant integer 42 and ignores time entirely; despite its name it does not read the clock.

## Behaviour

- now() -> 42
- takes no arguments
- always returns the same value on every call

## Terms

current time, timestamp, clock, time helper, date

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/store.py:14-15`
- **Signature:** `def now()`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-store-py]]
- **Called by:** [[session-init]], [[session-is-fresh]]
