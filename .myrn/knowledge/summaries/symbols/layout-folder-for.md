---
id: layout-folder-for
type: summary
status: unverified
title: layout::folder_for
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::folder_for
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::folder_for@549a5daca13970e1
map_version: '2'
---
# layout::folder_for

Maps a note type to the folder its note is written to. Returns None for any type not listed.

## Behaviour

- decision -> Some("decisions")
- root_cause or bug -> Some("bugs"); fact or feature -> Some("facts")
- unknown type such as "note" or "" -> None

## Terms

destination, storage location, where a note is saved, directory for type, type to directory, save path

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:19-28`
- **Signature:** `pub fn folder_for(ntype: &str) -> Option<&'static str>`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[engine-publish-plan]], [[engine-publish]], [[notes-write]]
- **Tests:** `every_type_has_a_folder`
