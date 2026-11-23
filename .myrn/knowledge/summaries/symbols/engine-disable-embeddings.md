---
id: engine-disable-embeddings
type: summary
status: unverified
title: Engine::disable_embeddings
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::disable_embeddings
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::disable_embeddings@bfe774b64f8cd146
map_version: '2'
---
# Engine::disable_embeddings

Sets embeddings false in project state and deletes every row from the vectors table. Model files are left in place.

## Behaviour

- Afterward embeddings_on is false and the vector count is 0
- Model files stay under data_home, so enabling again skips the download
- State is saved before the DELETE; if the DELETE fails, state is already false

## Terms

turn off, disable, clear vectors, remove embeddings, reset, wipe index

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:222-228`
- **Signature:** `pub fn disable_embeddings(&mut self) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[engine-state]], [[engine-save-state]]
- **Called by:** [[embeddings-disable]]
