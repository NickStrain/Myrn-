---
id: engine-write-note
type: summary
status: unverified
title: Engine::write_note
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::write_note
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::write_note@3f547afa48993153
map_version: '2'
---
# Engine::write_note

Writes a note file to the shared knowledge or private notes folder, then syncs the index.

## Behaviour

- The file is written before indexing, so it stays on disk if indexing fails
- A note without an id gets one from its content, and the writer checks existing ids
- Index sync runs with code_changed set to false

## Terms

save note, create note, private note, shared knowledge, note id, write file

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:297-311`
- **Signature:** `pub fn write_note(&mut self, note: &mut Note, visibility: Visibility) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-note-roots]], [[notes-write]], [[layout-knowledge-dir]], [[layout-notes-dir]], [[sync-sync]]
- **Tests:** `write_note_writes_first_then_indexes_and_shared_wins`
