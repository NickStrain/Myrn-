---
id: refresh
type: summary
status: unverified
title: refresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#refresh
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#refresh@8549771e7691a629
---
# refresh

Calls store.lookup(token) and returns undefined if nothing is found; otherwise builds a new Session from the looked-up value and returns it only if isFresh() is true, which almost always holds right after construction. Nothing is written to the store.

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:16-21`
- **Signature:** `function refresh(store: TokenStore, token: string): Session | undefined`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Calls:** [[session-3113]], [[session-isfresh]], [[tokenstore-lookup-4e6a]]
- **Tests:** `returns a session for a known token`
