---
id: seed-myrn-index-lang
type: summary
status: unverified
title: seed::myrn_index_lang
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::myrn_index_lang
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::myrn_index_lang@93dbf5d1ccb3fb08
map_version: '2'
---
# seed::myrn_index_lang

Returns true when the indexer has a language for the file extension of the given text.

## Behaviour

- main.rs -> true when Rust is an indexed language
- notes.txt -> false when no language matches the extension

## Terms

language extension, indexed file, file type, supported suffix

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:95-97`
- **Signature:** `fn myrn_index_lang(q: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[lang-from-path]]
- **Called by:** [[seed-find]]
