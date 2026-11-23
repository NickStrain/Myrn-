---
id: engine-data-home
type: summary
status: unverified
title: Engine::data_home
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::data_home
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::Engine::data_home@105ddb3481948803
map_version: '2'
---
# Engine::data_home

Returns the directory two levels above the project's data_dir, the shared root that holds all project data directories.

## Behaviour

- data_dir /x/data/proj -> /x
- fewer than two parents (e.g. 'proj') -> data_dir returned unchanged
- Doc comment says 'parent', but the code takes two parent() steps

## Terms

data root, shared directory, model location, grandparent path, home dir

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:41-48`
- **Signature:** `pub fn data_home(&self) -> PathBuf`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Calls:** [[sym-parent]]
- **Called by:** [[engine-embeddings-on]], [[engine-embed]], [[engine-enable-embeddings]], [[engine-embeddings-status]]
- **Tests:** `without_embeddings_nothing_changes`
