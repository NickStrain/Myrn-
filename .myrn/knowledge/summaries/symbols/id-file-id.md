---
id: id-file-id
type: summary
status: unverified
title: id::file_id
source: myrn summarize
about: sym:crates/myrn-model/src/id.rs#id::file_id
anchors:
- sym:crates/myrn-model/src/id.rs#id::file_id@5fa010d77f44111c
---
# id::file_id

Takes a path and returns `file:<path>`, with no normalization or validation of the input.

## Connections

- **Location:** `crates/myrn-model/src/id.rs:8-10`
- **Signature:** `pub fn file_id(path: &str) -> String`
- **In:** [[file-crates-myrn-model-src-id-rs]]
- **Called by:** [[engine-impact]], [[episode-persist]], [[indexer-persist]], [[symbols-resolve-all]], [[engine-map-body]], [[resolver-resolve-sync]]
