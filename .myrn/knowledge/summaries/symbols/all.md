---
id: all
type: summary
status: unverified
title: all
source: myrn summarize
about: sym:crates/myrn-adapters/src/lib.rs#all
anchors:
- sym:crates/myrn-adapters/src/lib.rs#all@911813f0ec4264d2
---
# all

Returns a Vec<Box<dyn Adapter>> containing every known adapter, currently ClaudeCode and Codex in that order; a new adapter must be added to this vec literal to be discoverable.

## Connections

- **Location:** `crates/myrn-adapters/src/lib.rs:16-18`
- **Signature:** `pub fn all() -> Vec<Box<dyn Adapter>>`
- **In:** [[file-crates-myrn-adapters-src-lib-rs]]
- **Called by:** [[by-name]], [[connect-choose]]
