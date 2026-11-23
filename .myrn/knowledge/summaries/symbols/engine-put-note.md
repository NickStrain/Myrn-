---
id: engine-put-note
type: summary
status: unverified
title: Engine::put_note
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::put_note
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::put_note@88da86923606e253
map_version: '2'
---
# Engine::put_note

Writes a note to disk. With a key it replaces the file at that key's path through a temp file and rename; without a key it creates a new file in the notes directory. It does not update the index.

## Behaviour

- key Some -> file at note_file(key) replaced via a temp file named with the process id, then renamed
- key None -> new file created in the notes directory, and its path returned
- a note id counts as known if it is in fresh or the notes table; that check is passed to rendering

## Terms

save, persist, write file, atomic write, temp file, rename, overwrite

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:386-408`
- **Signature:** `pub(crate) fn put_note( &self, note: &mut Note, key: Option<&str>, fresh: &HashSet<String>, ) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[engine-note-file]], [[frontmatter-render]], [[notes-write]], [[layout-notes-dir]]
- **Called by:** [[engine-record]], [[engine-forget]], [[engine-confirm]], [[engine-promote]]
