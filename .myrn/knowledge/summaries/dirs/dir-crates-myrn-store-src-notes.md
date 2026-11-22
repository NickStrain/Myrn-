---
id: dir-crates-myrn-store-src-notes
type: summary
status: unverified
title: crates/myrn-store/src/notes/
source: myrn summarize
about: module:crates/myrn-store/src/notes
anchors:
- module:crates/myrn-store/src/notes@4bfb9224dd18ce54
---
# crates/myrn-store/src/notes/

Markdown knowledge-note store: one note per file, named by content-hash id and Obsidian-compatible, and the source of truth for earned knowledge. mod.rs holds the write function and id/collision tests and re-exports Anchor, Evidence, Link, Note and Parsed; frontmatter.rs parses and renders the YAML header (parse never fails, returning defaults plus problems), layout.rs maps the note types to folders and builds Windows-safe file names, and wikilink.rs extracts [[wikilink]] targets from bodies as generic `related` edges.

## Connections

- **In:** [[dir-crates-myrn-store-src]]
- **Files:** [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
