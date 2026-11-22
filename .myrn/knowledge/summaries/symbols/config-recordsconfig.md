---
id: config-recordsconfig
type: summary
status: unverified
title: config::RecordsConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::RecordsConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::RecordsConfig@231c8065f767705a
---
# config::RecordsConfig

Controls learned records: whether the agent is asked to call `record` at session end after code edits, whether known bugs are recalled on tool failure, and the token budget for that recall (`failure_budget`).

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:57-66`
- **Signature:** `pub struct RecordsConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
