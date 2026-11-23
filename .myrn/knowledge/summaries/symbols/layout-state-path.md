---
id: layout-state-path
type: summary
status: unverified
title: Layout::state_path
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::state_path
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::state_path@4623ec76ea8886a9
map_version: '2'
---
# Layout::state_path

Returns the path to the per-user state file that stores consent and connected agents.

## Behaviour

- -> <data_dir>/state.json
- per the doc comment, state survives `rebuild`
- lives in the per-user data dir, not the repo

## Terms

consent, connected agents, state file, state json, survives rebuild

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:66-68`
- **Signature:** `pub fn state_path(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-state]], [[engine-save-state]]
