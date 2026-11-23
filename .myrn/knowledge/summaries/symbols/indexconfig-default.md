---
id: indexconfig-default
type: summary
status: unverified
title: IndexConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::IndexConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::IndexConfig::default@9dd115f68190a7bf
map_version: '2'
---
# IndexConfig::default

Sets the indexing defaults: a 1,000,000-byte file cap and 500 commits of history.

## Behaviour

- no input -> max_file_bytes 1000000
- no input -> history_depth 500

## Terms

index defaults, default file size cap, default history depth

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:152-157`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
