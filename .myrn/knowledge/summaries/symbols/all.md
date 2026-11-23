---
id: all
type: summary
status: unverified
title: all
source: myrn summarize
about: sym:crates/myrn-adapters/src/lib.rs#all
anchors:
- sym:crates/myrn-adapters/src/lib.rs#all@911813f0ec4264d2
map_version: '2'
---
# all

Returns a freshly built list of every known adapter as boxed trait objects, in a fixed order: ClaudeCode, then Codex.

## Behaviour

- no arguments -> Vec of 2 boxed adapters, ClaudeCode first and Codex second
- each call constructs new boxes, so nothing is cached between calls
- adding an adapter means editing the vec! literal here

## Terms

registry, adapter list, known agents, supported agents, all adapters

## Connections

- **Location:** `crates/myrn-adapters/src/lib.rs:16-18`
- **Signature:** `pub fn all() -> Vec<Box<dyn Adapter>>`
- **In:** [[file-crates-myrn-adapters-src-lib-rs]]
- **Called by:** [[by-name]], [[connect-choose]]
