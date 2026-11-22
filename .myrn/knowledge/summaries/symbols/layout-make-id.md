---
id: layout-make-id
type: summary
status: unverified
title: layout::make_id
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::make_id
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::make_id@080da283cadfdbc8
---
# layout::make_id

Builds an id of the form prefix-hex, where hex is the first len characters of the blake3 hash of the type and whitespace-normalized content (whitespace runs collapsed to single spaces, trimmed). len is clamped to 4 through 64, so smaller or larger values are adjusted. The same type and text always give the same id, and the type is part of the hash, so identical text under different types gets different ids.

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:49-53`
- **Signature:** `pub fn make_id(ntype: &str, content: &str, len: usize) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Calls:** [[layout-id-prefix]]
- **Called by:** [[engine-record]], [[notes-write]]
- **Tests:** `ids_are_stable_and_prefixed`, `id_collision_lengthens_the_id`
