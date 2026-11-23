---
id: walker-rust-use-paths
type: summary
status: unverified
title: Walker::rust_use_paths
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_use_paths
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_use_paths@e564cc9d64fedf1b
map_version: '2'
---
# Walker::rust_use_paths

Expands a Rust use tree into full (path, alias) pairs, handling nested lists, as renames, self leaves and * globs.

## Behaviour

- a::{B, c::D as E} gives ('a::B', 'B') and ('a::c::D', 'E')
- a::b::{self} gives ('a::b', 'b'); a::* gives ('a::*', '*')
- Unrecognised node kinds yield nothing

## Terms

use tree, use list, rename, glob import, use path, self import

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:751-801`
- **Signature:** `fn rust_use_paths(&self, n: Node<'_>, prefix: &str, out: &mut Vec<(String, String)>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[walker-field-text]], [[walker-rust-use-paths]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-use-paths]]
