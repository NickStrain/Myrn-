---
id: sync-sync
type: summary
status: unverified
title: sync::sync
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::sync
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::sync@0ece734c92355fd3
---
# sync::sync

Updates the notes table in one transaction. It deletes rows for vanished files, re-parses files whose size or mtime changed and whose content hash changed, and runs link when anything changed, relink is true, or the notes_linked meta key is missing. Files that fail to read are skipped and their existing rows are kept.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:124-202`
- **Signature:** `pub fn sync(conn: &mut Connection, roots: &[NoteRoot], relink: bool) -> Result<NotesReport>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[visibility-as-str]], [[frontmatter-parse]], [[sync-walk]], [[sync-content-hash]], [[sync-link]]
- **Called by:** [[engine-refresh]], [[engine-write-note]], [[engine-record]], [[engine-summarize]]
