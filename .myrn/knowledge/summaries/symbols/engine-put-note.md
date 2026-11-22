---
id: engine-put-note
type: summary
status: unverified
title: Engine::put_note
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::put_note
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::put_note@c183f0c8676fc301
---
# Engine::put_note

Writes a note and returns its path: with a key it overwrites that existing file atomically via a temp file and rename, otherwise it creates a new file in the notes directory. The caller must index the note afterwards.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:375-397`
- **Signature:** `fn put_note( &self, note: &mut Note, key: Option<&str>, fresh: &HashSet<String>, ) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[engine-note-file]], [[frontmatter-render]], [[notes-write]], [[layout-notes-dir]]
- **Called by:** [[engine-record]]
