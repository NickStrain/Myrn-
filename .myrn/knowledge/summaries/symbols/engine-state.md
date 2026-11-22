---
id: engine-state
type: summary
status: unverified
title: Engine::state
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::state
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::state@c6f917ff025ccc97
---
# Engine::state

Loads the private consent and connected-agent state from the layout state path.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:811-813`
- **Signature:** `pub fn state(&self) -> Result<myrn_store::State>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-state-path]], [[state-load]]
- **Called by:** [[engine-capture]], [[engine-status]], [[consent]], [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]], [[hook-handle]], [[log-run]]
