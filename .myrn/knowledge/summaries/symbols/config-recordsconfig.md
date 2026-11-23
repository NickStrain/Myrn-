---
id: config-recordsconfig
type: summary
status: unverified
title: config::RecordsConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::RecordsConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::RecordsConfig@231c8065f767705a
map_version: '2'
---
# config::RecordsConfig

Controls whether the agent is asked to record learnings at session end, and whether known bugs are injected on tool failure, with a token budget for that injection.

## Behaviour

- ask_at_session_end true -> agent is asked to call record after a turn that edited code
- recall_on_failure true -> a failing tool gets the matching recorded bug, cause and fix injected
- failure_budget 500 -> token cap on that injected recall

## Terms

learned records, session end prompt, failure recall, known bug injection, recording consent, token budget

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:115-124`
- **Signature:** `pub struct RecordsConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
