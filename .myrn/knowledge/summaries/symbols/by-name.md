---
id: by-name
type: summary
status: unverified
title: by_name
source: myrn summarize
about: sym:crates/myrn-adapters/src/lib.rs#by_name
anchors:
- sym:crates/myrn-adapters/src/lib.rs#by_name@040f15209bae6b80
---
# by_name

Returns the first adapter from all() whose name() equals the given &str, or None if no match; the lookup is exact and case-sensitive and builds a fresh set of boxed adapters on each call.

## Connections

- **Location:** `crates/myrn-adapters/src/lib.rs:20-22`
- **Signature:** `pub fn by_name(name: &str) -> Option<Box<dyn Adapter>>`
- **In:** [[file-crates-myrn-adapters-src-lib-rs]]
- **Calls:** [[all]]
- **Called by:** [[connect-choose]], [[disconnect-run]], [[doctor-agent-checks]], [[hook-handle]]
