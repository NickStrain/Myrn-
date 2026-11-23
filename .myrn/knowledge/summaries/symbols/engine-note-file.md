---
id: engine-note-file
type: summary
status: unverified
title: Engine::note_file
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::note_file
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::note_file@af12577eaf62edec
map_version: '2'
---
# Engine::note_file

Maps a notes-table key to a file path. Keys under the shared knowledge prefix go to the knowledge directory, and all other keys go to the notes directory.

## Behaviour

- shared prefix + /knowledge/x.md -> knowledge directory joined with x.md
- notes/a.md -> notes directory joined with a.md
- a.md with no prefix -> notes directory joined with a.md

## Terms

path lookup, key to path, knowledge dir, notes dir, file location, prefix strip

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:411-420`
- **Signature:** `pub(crate) fn note_file(&self, key: &str) -> PathBuf`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[layout-knowledge-dir]], [[layout-notes-dir]]
- **Called by:** [[engine-put-note]], [[engine-forget-plan]], [[engine-publish-plan]], [[engine-gc]]
