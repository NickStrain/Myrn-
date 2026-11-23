---
id: session-new
type: summary
status: unverified
title: Session::new
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::new
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session::new@ed2cbc7d0342c315
map_version: '2'
---
# Session::new

Builds a Session from a token string and stamps issued with store::now().

## Behaviour

- "abc" -> Session with token "abc" and issued = store::now()
- takes ownership of the String; no validation of token contents
- issued is always the current time, never a caller-supplied value

## Terms

create session, constructor, issue session, stamp time

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:9-11`
- **Signature:** `pub fn new(token: String) -> Session`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[store-now-store]]
- **Called by:** [[session-refresh-session]]
