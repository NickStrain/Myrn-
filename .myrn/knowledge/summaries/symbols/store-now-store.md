---
id: store-now-store
type: summary
status: unverified
title: store::now
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::now
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::now@0d309f24abd73728
map_version: '2'
---
# store::now

Returns the constant 42 as a u64. It does not read the clock despite its name.

## Behaviour

- No arguments -> always 42.
- Repeated calls always return the same value.
- The name suggests a timestamp, but the value is hard-coded.

## Terms

timestamp, current time, clock, time value, epoch, placeholder time

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:21-23`
- **Signature:** `pub fn now() -> u64`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Called by:** [[session-new]], [[session-is-fresh-session]]
