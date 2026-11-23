---
id: refresh
type: summary
status: unverified
title: refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#refresh
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#refresh@8549771e7691a629
map_version: '2'
---
# refresh

Looks up the token in the store; if found, builds a new Session from the found value and returns it only if it is fresh, otherwise returns undefined.

## Behaviour

- store.lookup returns undefined -> returns undefined and creates no Session
- store.lookup returns a value -> new Session built from that value, returned because it was just issued
- the Session token comes from the lookup result, not directly from the token argument

## Terms

renew session, reissue, revalidate, token lookup, session refresh, unknown token, expired token

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:16-21`
- **Signature:** `function refresh(store: TokenStore, token: string): Session | undefined`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Calls:** [[session-3113]], [[session-isfresh]], [[tokenstore-lookup-4e6a]]
- **Tests:** `returns a session for a known token`
