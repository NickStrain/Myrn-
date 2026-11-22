---
id: session-is-fresh-session
type: summary
status: unverified
title: Session::is_fresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::is_fresh
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::is_fresh@7887e839edc46889
---
# Session::is_fresh

Returns true when issued + 10 is greater than store::now(), so the session counts as fresh for 10 time units after issue. The 10 is hardcoded and the time unit is defined by store::now(), which is not shown here.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:13-15`
- **Signature:** `pub fn is_fresh(&self) -> bool`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[store-now-store]]
- **Called by:** [[session-refresh-session]]
