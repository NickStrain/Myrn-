---
id: sync-sync
type: summary
status: unverified
title: sync::sync
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::sync
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::sync@0ece734c92355fd3
map_version: '2'
---
# sync::sync

Brings the notes table up to date with the note files on disk in one transaction, then runs the link pass when needed.

## Behaviour

- rows whose file is gone are deleted; a file that cannot be read is kept and skipped
- a file whose size and mtime match the stored row is not read at all
- content unchanged but size or mtime changed: only size and mtime are updated; invalid UTF-8 adds problem 'not valid UTF-8; bad bytes replaced'

## Terms

reindex, update index, refresh notes, incremental scan, rebuild notes, sync notes

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:134-212`
- **Signature:** `pub fn sync(conn: &mut Connection, roots: &[NoteRoot], relink: bool) -> Result<NotesReport>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[visibility-as-str]], [[frontmatter-parse]], [[sync-walk]], [[sync-content-hash]], [[sync-link]]
- **Called by:** [[engine-refresh]], [[engine-write-note]], [[engine-record]], [[engine-forget]], [[engine-publish]], [[engine-summarize]], [[engine-gc]], [[engine-confirm]], [[engine-promote]]
