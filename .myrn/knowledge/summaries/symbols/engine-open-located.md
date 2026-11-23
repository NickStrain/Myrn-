---
id: engine-open-located
type: summary
status: unverified
title: Engine::open_located
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::open_located
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::open_located@b5befb4ae8aa655c
map_version: '2'
---
# Engine::open_located

Builds the layout, loads config from it, creates the database's parent directory and opens the database.

## Behaviour

- data_home None falls back to the default data home path
- last_refresh starts as None, so refresh_if_older refreshes on its first call

## Terms

database open, data directory, layout setup, store location

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:225-245`
- **Signature:** `fn open_located(loc: Located, data_home: Option<&Path>) -> Result<Engine>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[config-load]], [[sym-parent]], [[db-open]], [[paths-data-home]], [[layout-new]], [[layout-config-path]]
- **Called by:** [[engine-init]], [[engine-open]]
