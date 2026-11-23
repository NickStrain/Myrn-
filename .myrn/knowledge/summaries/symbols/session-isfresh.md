---
id: session-isfresh
type: summary
status: unverified
title: Session::isFresh
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.isFresh
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.isFresh@08ddb3d71798a12b
map_version: '2'
---
# Session::isFresh

Returns true if the session was issued less than 10 time units ago, measured by now(), and false otherwise.

## Behaviour

- issued 5 units ago -> true
- issued 10 or more units ago -> false (strict comparison)
- reads now() on every call, so the result changes as time passes

## Terms

expiry check, is valid, still valid, expired, not expired, freshness, staleness

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:10-12`
- **Signature:** `isFresh(): boolean`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Calls:** [[now]]
- **Called by:** [[refresh]]
