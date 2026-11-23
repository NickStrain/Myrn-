---
id: walker-rust-mod-path
type: summary
status: unverified
title: Walker::rust_mod_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_mod_path
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_mod_path@abb36f6fbb7eb71e
map_version: '2'
---
# Walker::rust_mod_path

For Rust, builds the module path at the current position from the file module prefix plus any inline mod blocks open around it.

## Behaviour

- Prefix parts followed by the names of enclosing inline mod blocks, joined with ::
- Rust code at top level with an empty prefix gives Some('') rather than None
- Non-Rust files -> None

## Terms

module path, module prefix, mod path, crate path, inline module, module location

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:190-202`
- **Signature:** `fn rust_mod_path(&self) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]]
- **Called by:** [[walker-push-import]], [[walker-push-def]]
