---
id: layout-file-name
type: summary
status: unverified
title: layout::file_name
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::file_name
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::file_name@b2880baaa8a3f031
map_version: '2'
---
# layout::file_name

Returns the note's on-disk name: the safe stem from `safe_stem` with ".md" appended. The reserved-name fix is applied before the extension is added.

## Behaviour

- bug-7f3a91 -> "bug-7f3a91.md"
- con -> "con_.md"
- empty id -> "_.md"

## Terms

markdown extension, .md suffix, on-disk name, note path, filename, extension

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:87-89`
- **Signature:** `pub fn file_name(id: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Calls:** [[layout-safe-stem]]
- **Called by:** [[engine-publish-plan]], [[engine-summarize]], [[notes-write]]
