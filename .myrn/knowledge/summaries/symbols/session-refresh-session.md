---
id: session-refresh-session
type: summary
status: unverified
title: session::refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::refresh
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::refresh@00915c0f5235174d
---
# session::refresh

Takes a &TokenStore and a &str token, returns None if store.lookup fails, otherwise builds a new Session from the found value and returns it as Some only if is_fresh is true. Since new sets issued to the current time, the is_fresh check should always pass right after construction, so the None branch is effectively unreachable with a monotonic clock. It does not modify the store.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:19-27`
- **Signature:** `pub fn refresh(store: &TokenStore, token: &str) -> Option<Session>`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[session-new]], [[session-is-fresh-session]], [[tokenstore-lookup-store]]
- **Tests:** `refresh_unknown_token`, `refreshes_known_token`
