---
id: session-3113
type: summary
status: unverified
title: Session
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#Session
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#Session@500c5b681d1f59b8
map_version: '2'
---
# Session

Holds a public token string and an issued timestamp taken from now() at construction, and reports whether it is still fresh.

## Behaviour

- new Session("abc") -> token is "abc" and issued is now()
- isFresh() is true while issued + 10 is greater than now()
- issued is a plain number field and can be read or overwritten directly

## Terms

session, auth token, issue time, timestamp, expiry window, time to live

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:3-13`
- **Signature:** `class Session`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Called by:** [[refresh]]
