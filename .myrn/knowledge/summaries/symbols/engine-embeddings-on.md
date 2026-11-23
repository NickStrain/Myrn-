---
id: engine-embeddings-on
type: summary
status: unverified
title: Engine::embeddings_on
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embeddings_on
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embeddings_on@f8a041671771ec31
map_version: '2'
---
# Engine::embeddings_on

Returns true only when the build supports embeddings, the model is installed under data_home, and this project's saved state has embeddings enabled.

## Behaviour

- Unsupported build or missing model -> false, and project state is not read (short-circuit)
- State read error -> false (unwrap_or)
- State says enabled but model files are missing -> false

## Terms

enabled check, feature flag, model installed, turned on, on or off, gate

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:51-55`
- **Signature:** `pub fn embeddings_on(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[supported]], [[installed]], [[engine-state]], [[engine-data-home]]
- **Called by:** [[engine-refresh]], [[engine-vector-seeds]]
- **Tests:** `without_embeddings_nothing_changes`
