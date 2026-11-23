---
id: indexer-parseresult
type: summary
status: unverified
title: indexer::ParseResult
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::ParseResult
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::ParseResult@dbeeabe880658420
map_version: '2'
---
# indexer::ParseResult

Private alias for the per-file outcome from parse_all: the file's stat, its content hash, and the parse Result.

## Behaviour

- an Err for one file does not affect the other entries
- Ok carries Parsed with defs, refs, imports and has_errors
- the hash is passed through unchanged for persist

## Terms

parse outcome, per-file result, tuple, type alias, parse output

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:155`
- **Signature:** `type ParseResult = (FileStat, String, Result<Parsed>);`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
