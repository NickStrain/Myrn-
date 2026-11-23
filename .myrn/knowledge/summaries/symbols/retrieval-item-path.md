---
id: retrieval-item-path
type: summary
status: unverified
title: retrieval::item_path
source: myrn summarize
about: sym:bench/src/retrieval.rs#retrieval::item_path
anchors:
- sym:bench/src/retrieval.rs#retrieval::item_path@03b59f811663f93e
map_version: '2'
---
# retrieval::item_path

Extracts the file path from a pack item id by dropping the optional prefix before the first colon and anything after the first hash.

## Behaviour

- sym:src/a.rs#foo -> src/a.rs
- file:src/a.rs -> src/a.rs; src/a.rs#foo with no prefix -> src/a.rs
- sym:a:b#c -> a:b (only the first colon is split); empty string -> empty string

## Terms

file path, item path, id prefix, strip symbol, path from id, hash suffix

## Connections

- **Location:** `bench/src/retrieval.rs:30-33`
- **Signature:** `fn item_path(id: &str) -> &str`
- **In:** [[file-bench-src-retrieval-rs]]
- **Called by:** [[retrieval-one]]
