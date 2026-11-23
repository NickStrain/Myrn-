---
id: layout-make-id
type: summary
status: unverified
title: layout::make_id
source: myrn summarize
about: sym:crates/myrn-store/src/notes/layout.rs#notes::layout::make_id
anchors:
- sym:crates/myrn-store/src/notes/layout.rs#notes::layout::make_id@080da283cadfdbc8
map_version: '2'
---
# layout::make_id

Builds an id of the form prefix-hash from the note type and content, using the first `len` hex digits of a blake3 hash. The content is whitespace-normalised before hashing, so equal knowledge gets the same id.

## Behaviour

- Runs of whitespace, including newlines, collapse to one space: "Refresh returns\n401" and "Refresh returns 401" give the same id
- len below 4 is raised to 4; len above 64 is capped at 64; the result is always 4 to 64 hex digits
- The type is hashed with the content, so the same text as fact and as decision gives different ids; root_cause uses the prefix rootcause

## Terms

content hash, deterministic id, stable id, dedupe key, fingerprint, hash length, whitespace normalisation

## Connections

- **Location:** `crates/myrn-store/src/notes/layout.rs:49-53`
- **Signature:** `pub fn make_id(ntype: &str, content: &str, len: usize) -> String`
- **In:** [[file-crates-myrn-store-src-notes-layout-rs]]
- **Calls:** [[layout-id-prefix]]
- **Called by:** [[engine-record]], [[notes-write]]
- **Tests:** `ids_are_stable_and_prefixed`, `id_collision_lengthens_the_id`
