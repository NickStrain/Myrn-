---
id: session-is-fresh
type: summary
status: unverified
title: Session::is_fresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.is_fresh
anchors:
- sym:tests/fixtures/repos/gate0/py/app/session.py#tests.fixtures.repos.gate0.py.app.session.Session.is_fresh@9b6c959365e4e7c0
map_version: '2'
---
# Session::is_fresh

Returns True when fewer than 10 units of now() have elapsed since issued, otherwise False.

## Behaviour

- issued + 10 > now() -> True; issued + 10 == now() -> False (boundary is exclusive)
- Reads the clock on each call, so the same session can become stale over time
- Does not modify the object; only compares timestamps

## Terms

stale check, still valid, not expired, age check, within window, time window, validity check

## Connections

- **Location:** `tests/fixtures/repos/gate0/py/app/session.py:9-10`
- **Signature:** `def is_fresh(self)`
- **In:** [[file-tests-fixtures-repos-gate0-py-app-session-py]]
- **Calls:** [[store-now]]
- **Called by:** [[session-refresh]]
