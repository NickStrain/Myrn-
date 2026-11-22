---
id: file-crates-myrn-store-src-notes-mod-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/mod.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/mod.rs
anchors:
- file:crates/myrn-store/src/notes/mod.rs@d993750e67ae6e5a
---
# crates/myrn-store/src/notes/mod.rs

Markdown notes store: one note per file named by content-hash id, Obsidian-compatible, and the source of truth for earned knowledge. Declares the frontmatter, layout, and wikilink submodules and re-exports Anchor, Evidence, Link, Note, and Parsed, and holds the write function plus tests for id generation and collision handling.

## Connections

- **Location:** `crates/myrn-store/src/notes/mod.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[notes-write]]
- **Imports:** [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-sync-rs]]
