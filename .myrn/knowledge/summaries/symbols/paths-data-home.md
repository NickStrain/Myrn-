---
id: paths-data-home
type: summary
status: unverified
title: paths::data_home
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::data_home
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::data_home@24412a3330a39e2a
map_version: '2'
---
# paths::data_home

Returns the per-user data root, taken from the MYRN_HOME env var if set, otherwise the platform local data directory plus `myrn`.

## Behaviour

- MYRN_HOME set -> that value as-is, with no existence or validity check, even if empty
- MYRN_HOME unset on Linux -> ~/.local/share/myrn
- no home directory found and MYRN_HOME unset -> Err "cannot find a home directory"

## Terms

data home, per-user data, MYRN_HOME override, local data dir, app data directory, user data path

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:15-22`
- **Signature:** `pub fn data_home() -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[real-main]], [[engine-open-located]]
