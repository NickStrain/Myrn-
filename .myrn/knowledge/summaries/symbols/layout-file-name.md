---
id: layout-file-name
type: summary
status: unverified
title: layout::file_name
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::file_name
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::file_name@b2880baaa8a3f031
---
# layout::file_name

Returns safe_stem(id) with ".md" appended. It performs no checks beyond what safe_stem does.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:87-89`
- **Signature:** `pub fn file_name(id: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Calls:** [[layout-safe-stem]]
- **Called by:** [[engine-summarize]], [[notes-write]]
