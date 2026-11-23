---
id: config-indexconfig
type: summary
status: unverified
title: config::IndexConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::IndexConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::IndexConfig@8815602811258369
map_version: '2'
---
# config::IndexConfig

Indexing limits from [index]: skips large files and caps how much git history is indexed.

## Behaviour

- file larger than max_file_bytes (default 1000000) -> not parsed
- history_depth 500 -> up to 500 commits of history indexed
- missing keys -> defaults

## Terms

file size limit, max file bytes, parse skip, git history depth, commit history, index limits

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:144-149`
- **Signature:** `pub struct IndexConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
