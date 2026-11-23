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

Maps note types to their storage folders and folder names back to types, builds content-hash note ids, and turns ids into file names that are valid on Windows and Linux. Main parts: the `NOTE_TYPES` list, the `folder_for` and `type_for_folder` lookups, the `id_prefix` and `make_id` id builders, the `RESERVED` device-name list, and `safe_stem` and `file_name`.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs`
- **In:** [[dir-crates-myrn-store-src-notes]]
- **Defines:** [[layout-folder-for]], [[layout-type-for-folder]], [[layout-id-prefix]], [[layout-make-id]], [[layout-safe-stem]], [[layout-file-name]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-store-src-notes-frontmatter-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
