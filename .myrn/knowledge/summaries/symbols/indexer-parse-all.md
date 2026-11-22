---
id: indexer-parse-all
type: summary
status: unverified
title: indexer::parse_all
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::parse_all
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::parse_all@02bc132f2c54173c
---
# indexer::parse_all

Parses files across up to available_parallelism threads, with at least 16 files per thread. Returns each file's stat, hash, and Result of parsing. Does not touch the database.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:158-193`
- **Signature:** `fn parse_all(files: Vec<(FileStat, Vec<u8>, String)>) -> Vec<ParseResult>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[fileparser-new]], [[fileparser-parse]]
- **Called by:** [[indexer-refresh]]
