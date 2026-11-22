---
id: store-now-store
type: summary
status: unverified
title: store::now
source: myrn summarize
about: sym:tests/fixtures/repos/gate0/src/store.rs#store::now
anchors:
- sym:tests/fixtures/repos/gate0/src/store.rs#store::now@0d309f24abd73728
---
# store::now

Takes no arguments and returns the hardcoded u64 value 42; does not read any clock despite the name.

## Connections

- **Location:** `tests/fixtures/repos/gate0/src/store.rs:21-23`
- **Signature:** `pub fn now() -> u64`
- **In:** [[file-tests-fixtures-repos-gate0-src-store-rs]]
- **Called by:** [[session-new]], [[session-is-fresh-session]]
