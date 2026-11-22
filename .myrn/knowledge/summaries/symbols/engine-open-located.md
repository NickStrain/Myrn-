---
id: engine-open-located
type: summary
status: unverified
title: Engine::open_located
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::open_located
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::open_located@b5befb4ae8aa655c
---
# Engine::open_located

Builds the Layout under the data home (paths::data_home unless overridden), creates the database parent directory, loads the config and opens the database.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:223-243`
- **Signature:** `fn open_located(loc: Located, data_home: Option<&Path>) -> Result<Engine>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[config-load]], [[sym-parent]], [[db-open]], [[paths-data-home]], [[layout-new]], [[layout-config-path]]
- **Called by:** [[engine-init]], [[engine-open]]
