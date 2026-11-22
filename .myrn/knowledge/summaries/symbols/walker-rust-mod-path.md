---
id: walker-rust-mod-path
type: summary
status: unverified
title: Walker::rust_mod_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_mod_path
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_mod_path@abb36f6fbb7eb71e
---
# Walker::rust_mod_path

Builds the Rust module path from the file prefix plus inline mod scopes joined by '::'; returns None for non-Rust files.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:190-202`
- **Signature:** `fn rust_mod_path(&self) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-family]]
- **Called by:** [[walker-push-import]], [[walker-push-def]]
