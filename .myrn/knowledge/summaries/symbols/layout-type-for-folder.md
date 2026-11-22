---
id: layout-type-for-folder
type: summary
status: unverified
title: layout::type_for_folder
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::type_for_folder
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::type_for_folder@844316d38d4d810b
---
# layout::type_for_folder

Gives the default type for a folder when a note's header omits one: decisions to decision, bugs to bug, workflows to workflow, facts to fact, summaries to summary. Returns None for an unknown folder. It cannot recover root_cause or feature, so callers must read the header when those matter.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:31-40`
- **Signature:** `pub fn type_for_folder(folder: &str) -> Option<&'static str>`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
