---
id: file-crates-myrn-store-src-notes-layout-rs
type: summary
status: unverified
title: crates/myrn-store/src/notes/layout.rs
source: myrn summarize
about: file:crates/myrn-store/src/notes/layout.rs
anchors:
- file:crates/myrn-store/src/notes/layout.rs@fb4a390c7903eda6
---
# crates/myrn-store/src/notes/layout.rs

Defines the note folder layout (decisions, bugs, workflows, facts, summaries) and the note type list, and builds note ids and Windows-safe file names. Main parts: the NOTE_TYPES list, the type-to-folder and folder-to-type maps, id construction, and the safe_stem and file_name sanitizer with its reserved device-name list.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[layout-folder-for]], [[layout-type-for-folder]], [[layout-id-prefix]], [[layout-make-id]], [[layout-safe-stem]], [[layout-file-name]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
