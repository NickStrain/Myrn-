---
id: adapter-detect
type: summary
status: unverified
title: Adapter::detect
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::detect
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::detect@85cf3181db77bbcd
map_version: '2'
---
# Adapter::detect

Reports whether the agent appears installed on this machine, and the evidence if so.

## Behaviour

- Some(reason) -> agent detected, reason explains the evidence
- None -> agent not detected

## Terms

installed check, agent presence, auto detect

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:73`
- **Signature:** `fn detect(&self) -> Option<String>;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
