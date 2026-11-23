---
id: file-crates-myrn-store-src-notes-wikilink-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/wikilink.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/wikilink.rs
anchors:
- file:crates/myrn-store/src/notes/wikilink.rs@2f0e3ac627e152e5
---
# crates/myrn-store/src/notes/wikilink.rs

Parses Obsidian-style [[wikilink]] forms out of note bodies: extract collects the link targets, normalize reduces raw link text to a bare note name, and unwrap_link strips [[ ]] from header values. Links inside fenced code blocks and inline code spans are ignored.

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[wikilink-extract]], [[wikilink-normalize]], [[wikilink-unwrap-link]]
- **Imported by:** [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
