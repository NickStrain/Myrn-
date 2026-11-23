---
id: session-session-session
type: summary
status: unverified
title: session::Session
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::Session@91553baf0df4ddee
map_version: '2'
---
# session::Session

Holds a session token string and the u64 timestamp it was issued at. Both fields are public.

## Behaviour

- Session { token, issued } -> direct struct literal, bypasses Session::new so issued is not set to now()
- fields are pub, so callers can read and overwrite token and issued directly

## Terms

session record, auth token, issue time, login session

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:3-6`
- **Signature:** `pub struct Session`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
