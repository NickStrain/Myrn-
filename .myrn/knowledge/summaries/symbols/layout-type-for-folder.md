---
id: layout-type-for-folder
type: summary
status: unverified
title: layout::type_for_folder
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::type_for_folder
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::type_for_folder@844316d38d4d810b
map_version: '2'
---
# layout::type_for_folder

Maps a folder name back to the default type for a note there whose header gives no type. Returns None for unknown folders.

## Behaviour

- bugs -> Some("bug"), so root_cause notes in bugs/ read back as bug
- facts -> Some("fact"), so a feature note in facts/ reads back as fact
- unknown folder such as "feature" or "" -> None

## Terms

default type, fallback type, inferred type, missing header type, reverse lookup, directory to type

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:31-40`
- **Signature:** `pub fn type_for_folder(folder: &str) -> Option<&'static str>`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
