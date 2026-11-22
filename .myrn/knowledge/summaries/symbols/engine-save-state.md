---
id: engine-save-state
type: summary
status: unverified
title: Engine::save_state
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::save_state
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::save_state@398998d100a36079
---
# Engine::save_state

Writes the consent and connected-agent state to the layout state path.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:815-817`
- **Signature:** `pub fn save_state(&self, state: &myrn_store::State) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-state-path]], [[state-save]]
- **Called by:** [[consent]], [[connect-run]], [[disconnect-run]], [[hook-handle]]
