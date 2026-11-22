---
id: layout-folder-for
type: summary
status: unverified
title: layout::folder_for
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::folder_for
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::folder_for@549a5daca13970e1
---
# layout::folder_for

Maps a note type to its folder: decision to decisions, bug and root_cause to bugs, workflow to workflows, fact and feature to facts, summary to summaries. Returns None for any other string. feature and fact share the facts folder, so the folder alone does not identify the type.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:19-28`
- **Signature:** `pub fn folder_for(ntype: &str) -> Option<&'static str>`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[notes-write]]
- **Tests:** `every_type_has_a_folder`
