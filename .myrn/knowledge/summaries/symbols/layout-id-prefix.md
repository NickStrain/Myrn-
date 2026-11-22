---
id: layout-id-prefix
type: summary
status: unverified
title: layout::id_prefix
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::id_prefix
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::id_prefix@2d752965834a2524
---
# layout::id_prefix

Returns the type name with underscores removed, so root_cause becomes rootcause and other names are unchanged. It does not validate the input.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:43-45`
- **Signature:** `pub fn id_prefix(ntype: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[layout-make-id]]
