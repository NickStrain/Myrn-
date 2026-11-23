---
id: file-crates-myrn-store-src-notes-mod-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/mod.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/mod.rs
anchors:
- file:crates/myrn-store/src/notes/mod.rs@1dcb64c7b057b347
---
# crates/myrn-store/src/notes/mod.rs

Markdown note storage: one .md file per record, named by content hash and kept in a per-type folder, so the files are the source of truth that sync indexes. Submodules are claims, directives, frontmatter, layout, and wikilink, and it re-exports Anchor, Evidence, Link, Note, and Parsed from frontmatter.

## Connections

- **Location:** `crates/myrn-store/src/notes/mod.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[notes-write]]
- **Imports:** [[file-crates-myrn-store-src-notes-claims-rs]], [[file-crates-myrn-store-src-notes-directives-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-viz-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-store-src-lib-rs]], [[file-crates-myrn-store-src-sync-rs]]
