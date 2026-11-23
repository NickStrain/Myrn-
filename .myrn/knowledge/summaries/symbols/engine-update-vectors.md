---
id: engine-update-vectors
type: summary
status: unverified
title: Engine::update_vectors
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::update_vectors
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::update_vectors@30185f32c606bdf9
map_version: '2'
---
# Engine::update_vectors

Deletes vectors whose node id no longer exists, then embeds every non-const/static symbol, file, and earned non-episode node whose stored hash differs, in batches of 256 with one transaction per batch.

## Behaviour

- Text is 'kind qname-words. signature summary-or-doc', cut to 1500 chars; files use 'file path-words. summary'; other types use 'type title. summary'
- The hash is the first 16 hex chars of blake3 over MODEL_REPO plus newline plus text, so changing the model re-embeds everything
- If the model returns None for a batch, it bails; earlier batches stay committed and the stale-row delete has already run

## Terms

incremental reindex, stale vectors, hash change, batch insert, re-embed, vector cleanup, refresh embeddings

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:70-151`
- **Signature:** `pub fn update_vectors(&mut self) -> Result<VectorReport>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[to-bytes]], [[engine-embed]], [[fts-split-identifier]]
- **Called by:** [[engine-refresh]], [[engine-enable-embeddings]]
