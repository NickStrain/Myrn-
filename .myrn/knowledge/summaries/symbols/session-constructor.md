---
id: session-constructor
type: summary
status: unverified
title: Session::constructor
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.constructor
anchors:
- sym:tests/fixtures/repos/gate0/web/src/session.ts#Session.constructor@1a0af4e976ad3abd
map_version: '2'
---
# Session::constructor

Stores the token parameter as a public property and sets issued to the current time.

## Behaviour

- new Session("t") -> token "t", issued set to now()
- empty string token is stored as "" with no validation
- issued is taken once at construction and never updated here

## Terms

create session, issue timestamp, wrap token, initialise, new session

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/session.ts:6-8`
- **Signature:** `constructor(public token: string)`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-session-ts]]
- **Calls:** [[now]]
