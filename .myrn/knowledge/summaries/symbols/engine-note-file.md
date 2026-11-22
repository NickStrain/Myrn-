---
id: engine-note-file
type: summary
status: unverified
title: Engine::note_file
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::note_file
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::note_file@af12577eaf62edec
---
# Engine::note_file

Maps a notes-table key to a filesystem path: keys under the shared knowledge prefix go to the knowledge directory, all other keys go to the notes directory with any notes/ prefix removed.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:400-409`
- **Signature:** `pub(crate) fn note_file(&self, key: &str) -> PathBuf`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]]
- **Called by:** [[engine-put-note]]
