---
id: layout-id-prefix
type: summary
status: unverified
title: layout::id_prefix
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::id_prefix
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::id_prefix@2d752965834a2524
map_version: '2'
---
# layout::id_prefix

Returns the id prefix for a note type by removing every underscore. Other characters are kept as they are.

## Behaviour

- root_cause -> "rootcause"
- bug -> "bug" (unchanged)
- a_b_c -> "abc" (all underscores removed)

## Terms

id stem, id head, identifier prefix, underscore removal, type tag

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:43-45`
- **Signature:** `pub fn id_prefix(ntype: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[layout-make-id]]
