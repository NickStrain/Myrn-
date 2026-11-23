---
id: dir-crates-myrn-store-src-notes
type: summary
status: unverified
title: crates/myrn-store/src/notes/
source: myrn summarize
about: module:crates/myrn-store/src/notes
anchors:
- module:crates/myrn-store/src/notes@d337b053fbe98fd3
---
# crates/myrn-store/src/notes/

Markdown note storage for the myrn-store crate: mod.rs keeps one .md file per record in a per-type folder, named by content hash, and those files are the source of truth that sync indexes. layout.rs maps note types to folders and builds Windows- and Linux-safe file names, frontmatter.rs parses and writes the typed YAML header, and wikilink.rs extracts [[links]] from bodies, while directives.rs strips directive-shaped text and claims.rs flags opposing statements when records are published or linked.

## Connections

- **In:** [[dir-crates-myrn-store-src]]
- **Files:** [[file-crates-myrn-store-src-notes-claims-rs]], [[file-crates-myrn-store-src-notes-directives-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-layout-rs]], [[file-crates-myrn-store-src-notes-mod-rs]], [[file-crates-myrn-store-src-notes-wikilink-rs]]
