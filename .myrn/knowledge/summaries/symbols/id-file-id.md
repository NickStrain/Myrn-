---
id: id-file-id
type: summary
status: unverified
title: id::file_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::file_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::file_id@5fa010d77f44111c
map_version: '2'
---
# id::file_id

Builds a file node id by prefixing the repo-relative path with `file:`.

## Behaviour

- "src/a.rs" -> "file:src/a.rs"
- "" -> "file:" (no validation; empty path is not rejected)

## Terms

file node, file identifier, path id, file key

## Connections

- **Location:** `crates/myrn-model/src/id.rs:8-10`
- **Signature:** `pub fn file_id(path: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-impact]], [[episode-persist]], [[indexer-persist]], [[symbols-resolve-all]], [[engine-map-body]], [[engine-viz-map]], [[resolver-resolve-sync]]
