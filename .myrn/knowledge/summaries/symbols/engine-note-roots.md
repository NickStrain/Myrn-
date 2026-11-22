---
id: engine-note-roots
type: summary
status: unverified
title: Engine::note_roots
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::note_roots
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::note_roots@a23fa3ec8ab64138
---
# Engine::note_roots

Returns the two note directories, shared knowledge first so that a shared note wins over a private note with the same id, then private notes.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:272-285`
- **Signature:** `pub(crate) fn note_roots(&self) -> Vec<sync::NoteRoot>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]]
- **Called by:** [[engine-refresh]], [[engine-write-note]], [[engine-record]], [[engine-summarize]]
