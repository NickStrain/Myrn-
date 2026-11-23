---
id: indexer-parse-all
type: summary
status: unverified
title: indexer::parse_all
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::parse_all
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::parse_all@02bc132f2c54173c
map_version: '2'
---
# indexer::parse_all

Parses each (stat, bytes, hash) on scoped threads, splitting the list into chunks of at least 16 files, and returns results in chunk order.

## Behaviour

- empty input -> empty Vec, no threads spawned
- 16 or fewer files -> one thread
- a thread that panics drops its files from the output with no error entry

## Terms

multithreaded, parallel, concurrency, worker threads, batch, cores

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:158-193`
- **Signature:** `fn parse_all(files: Vec<(FileStat, Vec<u8>, String)>) -> Vec<ParseResult>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[fileparser-new]], [[fileparser-parse]]
- **Called by:** [[indexer-refresh]]
