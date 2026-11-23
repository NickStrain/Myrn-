---
id: engine-state
type: summary
status: unverified
title: Engine::state
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::state
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::state@c6f917ff025ccc97
map_version: '2'
---
# Engine::state

Loads the private state file, which holds consent and connected agents.

## Behaviour

- Errors if the state file cannot be loaded

## Terms

consent, connected agents, private settings, agent list

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:877-879`
- **Signature:** `pub fn state(&self) -> Result<myrn_store::State>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-state-path]], [[state-load]]
- **Called by:** [[engine-capture]], [[engine-status]], [[engine-embeddings-on]], [[engine-enable-embeddings]], [[engine-disable-embeddings]], [[engine-embeddings-status]], [[consent]], [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]], [[hook-handle]], [[log-run]]
