---
id: now
type: summary
status: unverified
title: now
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/web/src/store.ts#now
anchors:
- sym:tests/fixtures/repos/gate0/web/src/store.ts#now@c2cae9e5de476a9b
map_version: '2'
---
# now

Returns the number 42 as a fixed placeholder; it does not read the clock despite its name.

## Behaviour

- now() -> 42 on every call
- takes no arguments
- not a timestamp, so do not rely on it for time

## Terms

current time, timestamp, clock, time helper, stub

## Connections

- **Location:** `tests/fixtures/repos/gate0/web/src/store.ts:14-16`
- **Signature:** `function now(): number`
- **In:** [[file-tests-fixtures-repos-gate0-web-src-store-ts]]
- **Called by:** [[session-constructor]], [[session-isfresh]]
