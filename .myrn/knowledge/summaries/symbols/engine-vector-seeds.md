---
id: engine-vector-seeds
type: summary
status: unverified
title: Engine::vector_seeds
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::vector_seeds
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::vector_seeds@9bda3283e2e91681
map_version: '2'
---
# Engine::vector_seeds

Embeds the task, scores every stored vector by dot product, keeps hits above max(min_similarity, 20th-best score + margin), caps at top_k, and maps each score to a Seed weight up to 6.0 tagged 'similar meaning'.

## Behaviour

- Embeddings off, task embed fails, or all-zero query -> empty Vec
- Fewer than 20 vectors -> the lowest stored score is the background; with no vectors -> empty Vec
- Seed score = min(3 + 20 * (similarity - floor), 6.0), so hits just above the floor get about 3

## Terms

semantic seeds, similarity search, meaning match, nearest neighbours, vector retrieval, top k, similarity floor

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:154-196`
- **Signature:** `pub fn vector_seeds(&self, task: &str) -> Result<Vec<Seed>>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[dot]], [[from-bytes]], [[engine-embeddings-on]], [[engine-embed]]
- **Called by:** [[engine-context-excluding]]
- **Tests:** `without_embeddings_nothing_changes`
