---
id: by-name
type: summary
status: unverified
title: by_name
source: myrn summarize
about: sym:crates/myrn-adapters/src/lib.rs#by_name
anchors:
- sym:crates/myrn-adapters/src/lib.rs#by_name@040f15209bae6b80
map_version: '2'
---
# by_name

Looks up one adapter by its name() string by scanning all() and returning the first exact match, or None.

## Behaviour

- name that equals an adapter's name() -> Some(that adapter)
- unknown name or empty string -> None, unless an adapter's name() is empty
- comparison is exact and case-sensitive (==), with no trimming or normalization

## Terms

lookup, find adapter, by name, agent name, adapter registry lookup

## Connections

- **Location:** `crates/myrn-adapters/src/lib.rs:20-22`
- **Signature:** `pub fn by_name(name: &str) -> Option<Box<dyn Adapter>>`
- **In:** [[file-crates-myrn-adapters-src-lib-rs]]
- **Calls:** [[all]]
- **Called by:** [[connect-choose]], [[disconnect-run]], [[doctor-agent-checks]], [[hook-handle]]
