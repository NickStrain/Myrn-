---
id: session-refresh-session
type: summary
status: unverified
title: session::refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::refresh
anchors:
- sym:tests/fixtures/repos/gate0/src/auth/session.rs#auth::session::refresh@00915c0f5235174d
map_version: '2'
---
# session::refresh

Looks up a token in the store and, if found and the new session is fresh, returns Some(Session). Otherwise returns None.

## Behaviour

- unknown token -> None, via the ? on store.lookup
- known token -> new Session created with issued = now, so is_fresh is true and result is Some
- the is_fresh check always passes on a newly built session, so the None branch after the lookup is effectively unreachable

## Terms

renew session, reissue token, token lookup, session renewal, re-authenticate

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/auth/session.rs:19-27`
- **Signature:** `pub fn refresh(store: &TokenStore, token: &str) -> Option<Session>`
- **In:** [[file-tests-fixtures-repos-gate0-src-auth-session-rs]]
- **Calls:** [[session-new]], [[session-is-fresh-session]], [[tokenstore-lookup-store]]
- **Tests:** `refresh_unknown_token`, `refreshes_known_token`
