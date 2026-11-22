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

Extracts Obsidian-style [[wikilink]] targets from note bodies for indexing as generic `related` edges, skipping fenced code blocks and inline code spans. Its main parts are `extract` (body to ordered, deduplicated targets), `normalize` (raw link text to bare target name), and `unwrap_link` (header value to target name).

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[wikilink-extract]], [[wikilink-normalize]], [[wikilink-unwrap-link]]
- **Imported by:** [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-sync-rs]]
