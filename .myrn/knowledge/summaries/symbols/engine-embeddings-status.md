---
id: engine-embeddings-status
type: summary
status: unverified
title: Engine::embeddings_status
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embeddings_status
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embeddings_status@21d5e3024ac232b6
map_version: '2'
---
# Engine::embeddings_status

Builds an EmbeddingsStatus by reading project state, checking support and installation, counting vectors, and probing the helper with one empty-string request.

## Behaviour

- Project state read error -> the whole call returns Err (via ?)
- helper_running is true only when the probe request returns Some
- vectors is counted even when embeddings are disabled

## Terms

status report, health check, helper probe, model presence, vector count, diagnostics

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:230-244`
- **Signature:** `pub fn embeddings_status(&self) -> Result<EmbeddingsStatus>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[client-request]], [[model-dir]], [[supported]], [[installed]], [[engine-state]], [[engine-data-home]]
- **Called by:** [[embeddings-status]]
- **Tests:** `without_embeddings_nothing_changes`
