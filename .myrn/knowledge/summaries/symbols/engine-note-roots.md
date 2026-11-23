---
id: engine-note-roots
type: summary
status: unverified
title: Engine::note_roots
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::note_roots
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::note_roots@a23fa3ec8ab64138
map_version: '2'
---
# Engine::note_roots

Lists the two note directories, shared knowledge first and private notes second.

## Behaviour

- Shared root is the knowledge folder, private root is the notes folder
- Order matters: a shared note wins over a private note with the same id

## Terms

note directories, note sources, visibility, shared knowledge, private notes

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:280-293`
- **Signature:** `pub(crate) fn note_roots(&self) -> Vec<sync::NoteRoot>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]]
- **Called by:** [[engine-refresh]], [[engine-write-note]], [[engine-record]], [[engine-forget]], [[engine-publish]], [[engine-summarize]], [[engine-gc]], [[engine-confirm]], [[engine-promote]]
