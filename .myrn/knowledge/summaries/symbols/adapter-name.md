---
id: adapter-name
type: summary
status: unverified
title: Adapter::name
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::name
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::Adapter::name@86d515e452e49702
map_version: '2'
---
# Adapter::name

Returns the short static id of the agent, used on the command line and in hook commands.

## Behaviour

- Examples given in the doc: 'claude', 'codex'
- Return type is &'static str, so no allocation
- Exact value is set by each implementation

## Terms

agent id, short name, cli id, command id

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:70`
- **Signature:** `fn name(&self) -> &'static str;`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
