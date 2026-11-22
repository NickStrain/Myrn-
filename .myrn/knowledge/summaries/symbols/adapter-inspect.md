---
id: adapter-inspect
type: summary
status: unverified
title: Adapter::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::inspect
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::inspect@6f0946cf92742aa9
---
# Adapter::inspect

Compares on-disk state with what install would write and returns one Finding per checked piece.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:79`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
