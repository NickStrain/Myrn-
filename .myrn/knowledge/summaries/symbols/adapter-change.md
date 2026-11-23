---
id: adapter-change
type: summary
status: unverified
title: adapter::Change
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Change
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Change@00c2647b56896768
map_version: '2'
---
# adapter::Change

Holds one human-facing record of a configuration edit: the path and a description of what changed.

## Behaviour

- path -> stored as supplied by the adapter, not normalised here
- what -> free-text description shown to the developer

## Terms

config edit, changed file, install report, user visible change

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:45-48`
- **Signature:** `pub struct Change`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
