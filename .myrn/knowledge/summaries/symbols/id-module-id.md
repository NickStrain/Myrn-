---
id: id-module-id
type: summary
status: unverified
title: id::module_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::module_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::module_id@f7a33e23f5cc85e0
map_version: '2'
---
# id::module_id

Builds a module node id from a directory, mapping the empty string to the repository root id `module:.`.

## Behaviour

- "" -> "module:." (repository root)
- "src/a" -> "module:src/a"
- "." -> "module:." (passed through unchanged, so it matches the root id by coincidence)

## Terms

directory id, package node, folder id, root module, repo root

## Connections

- **Location:** `crates/myrn-model/src/id.rs:13-19`
- **Signature:** `pub fn module_id(dir: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-brief]], [[indexer-persist]], [[engine-map-body]], [[engine-viz-map]], [[resolver-resolve-sync]]
- **Tests:** `ids`
