---
id: engine-save-state
type: summary
status: unverified
title: Engine::save_state
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::save_state
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::save_state@398998d100a36079
map_version: '2'
---
# Engine::save_state

Writes the state to the private state file, replacing the whole file.

## Behaviour

- Overwrites the previous state completely

## Terms

persist consent, save settings, write state

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:881-883`
- **Signature:** `pub fn save_state(&self, state: &myrn_store::State) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-state-path]], [[state-save]]
- **Called by:** [[engine-enable-embeddings]], [[engine-disable-embeddings]], [[consent]], [[connect-run]], [[disconnect-run]], [[hook-handle]]
