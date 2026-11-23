---
id: embeddings-embeddingsstatus
type: summary
status: unverified
title: embeddings::EmbeddingsStatus
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::EmbeddingsStatus
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::EmbeddingsStatus@84aacec2f65a8032
map_version: '2'
---
# embeddings::EmbeddingsStatus

Snapshot for status display: feature support, project enabled flag, model installed, model repo and directory, stored vector count, and whether the helper answers.

## Behaviour

- enabled comes from project state; installed checks model files under data_home
- vectors is COUNT(*) of the vectors table, regardless of the enabled flag
- helper_running is true only if a request containing one empty string returns a result

## Terms

status, health, model presence, helper alive, vector count, feature support

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:29-37`
- **Signature:** `pub struct EmbeddingsStatus`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
