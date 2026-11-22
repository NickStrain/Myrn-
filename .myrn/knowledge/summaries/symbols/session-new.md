---
id: session-new
type: summary
status: unverified
title: Session::new
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::new
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::new@ed2cbc7d0342c315
---
# Session::new

Takes an owned token String and returns a Session with issued set to store::now() at construction time. Reads the clock but has no other side effects.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:9-11`
- **Signature:** `pub fn new(token: String) -> Session`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[store-now-store]]
- **Called by:** [[session-refresh-session]]
