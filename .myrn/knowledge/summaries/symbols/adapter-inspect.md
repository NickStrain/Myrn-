---
id: adapter-inspect
type: summary
status: unverified
title: Adapter::inspect
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::inspect
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::inspect@6f0946cf92742aa9
map_version: '2'
---
# Adapter::inspect

Compares what is installed with what install would write for the same ctx and mode, and returns one finding per checked piece.

## Behaviour

- Same ctx and mode as install -> findings about the current state
- Returns a Vec<Finding>, one entry per checked piece

## Terms

doctor, health check, drift, verify, compare config

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:79`
- **Signature:** `fn inspect(&self, ctx: &InstallCtx, mode: Mode) -> Vec<Finding>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
