---
id: notes-write
type: summary
status: unverified
title: notes::write
source: myrn summarize
about: sym:crates/myrn-store/src/notes/mod.rs#notes::write
anchors:
- sym:crates/myrn-store/src/notes/mod.rs#notes::write@fdf7a7c231a7b456
---
# notes::write

Writes a note into the folder for its type under root and returns the path. If the note has an empty id, it derives one from the type and body, starting at 6 hex chars and growing by 2 until no other note with different body text holds that id (capped at 64), then writes atomically via a temp file and rename. Errors if the note type has no folder; creates the folder if needed, and overwrites an existing file with the same id.

## Connections

- **Location:** `crates/myrn-store/src/notes/mod.rs:23-58`
- **Signature:** `pub fn write(root: &Path, note: &mut Note, note_ids: &dyn Fn(&str) -> bool) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-notes-mod-rs]]
- **Calls:** [[frontmatter-parse]], [[frontmatter-render]], [[layout-folder-for]], [[layout-make-id]], [[layout-safe-stem]], [[layout-file-name]]
- **Called by:** [[engine-write-note]], [[engine-put-note]]
- **Tests:** `writes_into_type_folder_with_content_id`, `id_collision_lengthens_the_id`
