---
id: engine-write-note
type: summary
status: unverified
title: Engine::write_note
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::write_note
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::write_note@3f547afa48993153
---
# Engine::write_note

Writes the note file to the shared knowledge or private notes directory, generating an id from the content when none is set, then syncs the index and returns the path. The file is always written before indexing.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:289-303`
- **Signature:** `pub fn write_note(&mut self, note: &mut Note, visibility: Visibility) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-note-roots]], [[notes-write]], [[layout-knowledge-dir]], [[layout-notes-dir]], [[sync-sync]]
- **Tests:** `write_note_writes_first_then_indexes_and_shared_wins`
