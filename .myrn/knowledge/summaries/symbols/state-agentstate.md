---
id: state-agentstate
type: summary
status: unverified
title: state::AgentState
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::AgentState
anchors:
- sym:crates/myrn-store/src/state.rs#state::AgentState@9f9edb443ddc2edc
map_version: '2'
---
# state::AgentState

Records one connected agent: its mode, when it connected, and which files myrn wrote or created for it.

## Behaviour

- mode is a free String; the code only documents "hooks" or "pull_only" and does not enforce them
- created is optional on load (serde default), so older entries without it deserialize with an empty list
- the doc comment says created files are removed on disconnect when empty, but no removal logic is in this file

## Terms

connected agent, agent record, adapter state, hooks mode, pull only mode, written files, created files, instruction files, disconnect cleanup, connected time

## Connections

- **Location:** `crates/myrn-store/src/state.rs:30-39`
- **Signature:** `pub struct AgentState`
- **In:** [[file-crates-myrn-store-src-state-rs]]
