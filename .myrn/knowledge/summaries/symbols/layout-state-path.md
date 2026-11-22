---
id: layout-state-path
type: summary
status: unverified
title: Layout::state_path
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::Layout::state_path
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::Layout::state_path@4623ec76ea8886a9
---
# Layout::state_path

Returns data_dir/state.json, which stores consent and connected agents. The source comment says it survives rebuild.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:66-68`
- **Signature:** `pub fn state_path(&self) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-state]], [[engine-save-state]]
