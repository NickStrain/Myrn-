---
id: engine-embed
type: summary
status: unverified
title: Engine::embed
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embed
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::embed@d7f76b4d170e2239
map_version: '2'
---
# Engine::embed

Sends texts to the embedding helper for this data_home, passing the helper executable when available, and returns one f32 vector per text or None.

## Behaviour

- Helper exe None -> the client loads the model itself, per the helper_exe doc
- Model unavailable -> None; callers decide whether that is an error
- No length or empty-slice check happens in this method; input is passed through

## Terms

embed call, vectorize, text to vector, model call, inference, helper request

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:65-67`
- **Signature:** `pub fn embed(&self, texts: &[String]) -> Option<Vec<Vec<f32>>>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[client-embed]], [[engine-data-home]], [[engine-helper-exe]]
- **Called by:** [[main-simcheck]], [[engine-update-vectors]], [[engine-vector-seeds]]
