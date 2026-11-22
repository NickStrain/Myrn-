---
id: paths-data-home
type: summary
status: unverified
title: paths::data_home
source: myrn summarize
about: sym:crates/myrn-store/src/paths.rs#paths::data_home
anchors:
- sym:crates/myrn-store/src/paths.rs#paths::data_home@24412a3330a39e2a
---
# paths::data_home

Returns the per-user data root from the MYRN_HOME environment variable if set, otherwise the platform local data directory joined with myrn. Returns an error only when MYRN_HOME is unset and no home directory can be found.

## Connections

- **Location:** `crates/myrn-store/src/paths.rs:15-22`
- **Signature:** `pub fn data_home() -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-paths-rs]]
- **Called by:** [[engine-open-located]]
