---
id: session-is-fresh-session
type: summary
status: unverified
title: Session::is_fresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::is_fresh
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::is_fresh@7887e839edc46889
map_version: '2'
---
# Session::is_fresh

Returns true while the session is less than 10 time units old, measured against store::now().

## Behaviour

- issued = now -> true (10 > 0)
- issued = now - 9 -> true; issued = now - 10 -> false (boundary is exclusive)
- issued in the future -> true, since the comparison only checks the upper bound

## Terms

expiry check, session age, still valid, ttl, stale session

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:13-15`
- **Signature:** `pub fn is_fresh(&self) -> bool`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[store-now-store]]
- **Called by:** [[session-refresh-session]]
