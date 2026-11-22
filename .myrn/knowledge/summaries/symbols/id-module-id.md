---
id: id-module-id
type: summary
status: unverified
title: id::module_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::module_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::module_id@f7a33e23f5cc85e0
---
# id::module_id

Takes a directory and returns `module:<dir>`; an empty string maps to `module:.` so the repository root has a fixed id.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:13-19`
- **Signature:** `pub fn module_id(dir: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-brief]], [[indexer-persist]], [[engine-map-body]], [[resolver-resolve-sync]]
- **Tests:** `ids`
