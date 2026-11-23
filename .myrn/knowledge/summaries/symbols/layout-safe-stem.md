---
id: layout-safe-stem
type: summary
status: unverified
title: layout::safe_stem
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::safe_stem
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::safe_stem@24eea8a4c01cb699
map_version: '2'
---
# layout::safe_stem

Turns an id into a file stem that is valid on every OS. It lowercases the id, replaces control characters and <>:"/\|?* with underscores, strips trailing dots and spaces, and appends an underscore to Windows device names.

## Behaviour

- Bug-7F3A91 -> "bug-7f3a91"
- con -> "con_"; aux.notes -> "aux_.notes" (the device check uses the part before the first dot); a:b/c? -> "a_b_c_"
- trailing. -> "trailing"; empty input -> "_"; console -> "console" (not a device name)

## Terms

sanitise, sanitize filename, illegal characters, reserved name, device name, portable filename, escape, lowercase

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:62-85`
- **Signature:** `pub fn safe_stem(id: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Called by:** [[engine-publish]], [[frontmatter-parse]], [[layout-file-name]], [[notes-write]]
- **Tests:** `windows_safe_names`
