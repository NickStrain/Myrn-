---
id: layout-safe-stem
type: summary
status: unverified
title: layout::safe_stem
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::safe_stem
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::safe_stem@24eea8a4c01cb699
---
# layout::safe_stem

Lowercases the id, replaces control characters and the characters < > : " / \ | ? * with underscores, strips trailing dots and spaces, and turns an empty result into an underscore. If the part before the first dot is a Windows reserved name (con, prn, aux, nul, com1-9, lpt1-9), it inserts an underscore after that name, so con becomes con_ and aux.notes becomes aux_.notes. Names such as console are left alone.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:62-85`
- **Signature:** `pub fn safe_stem(id: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[frontmatter-parse]], [[layout-file-name]], [[notes-write]]
- **Tests:** `windows_safe_names`
