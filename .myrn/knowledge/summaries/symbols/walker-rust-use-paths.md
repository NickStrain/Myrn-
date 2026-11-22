---
id: walker-rust-use-paths
type: summary
status: unverified
title: Walker::rust_use_paths
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_use_paths
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_use_paths@e564cc9d64fedf1b
---
# Walker::rust_use_paths

Flattens a Rust use tree into (path, alias) pairs, handling `as` renames, nested lists, `self` segments, and `*` globs (alias "*").

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:751-801`
- **Signature:** `fn rust_use_paths(&self, n: Node<'_>, prefix: &str, out: &mut Vec<(String, String)>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-rust-use-paths]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-use-paths]]
