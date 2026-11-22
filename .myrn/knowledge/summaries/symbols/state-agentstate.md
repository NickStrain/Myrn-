---
id: state-agentstate
type: summary
status: unverified
title: state::AgentState
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::AgentState
anchors:
- sym:crates/myrn-store/src/state.rs#state::AgentState@9f9edb443ddc2edc
---
# state::AgentState

Connection record for one adapter: mode is "hooks" or "pull_only", connected_time is Unix seconds, written lists files myrn wrote or changed, and created lists instruction files myrn created rather than edited. Disconnect removes files in created only if they are left empty.

## Connections

- **Location:** `crates/myrn-store/src/state.rs:27-36`
- **Signature:** `pub struct AgentState`
- **In:** [[file-crates-myrn-store-src-state-rs]]
