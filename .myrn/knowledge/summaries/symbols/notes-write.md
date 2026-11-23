---
id: notes-write
type: summary
status: unverified
title: notes::write
source: myrn summarize
about: sym:crates/myrn-store/src/notes/mod.rs#notes::write
anchors:
- sym:crates/myrn-store/src/notes/mod.rs#notes::write@fdf7a7c231a7b456
map_version: '2'
---
# notes::write

Writes a note as a markdown file into the folder for its type under root, creating the folder if needed, and returns the path. If the note has no id, it derives one from the note type and body, lengthening it on collision with a different body.

## Behaviour

- note.ntype 'root_cause' with empty id -> id starts 'rootcause-', file at root/bugs/<id>.md, note.id set
- unknown note.ntype -> error 'unknown note type' and nothing is written
- empty id, existing file with same id but different body words -> id grows by 2 hex chars (from 6, max 64); at 64 it writes anyway

## Terms

save, persist, content hash id, id collision, note folder, overwrite, atomic rename, markdown file

## Connections

- **Location:** `crates/myrn-store/src/notes/mod.rs:25-60`
- **Signature:** `pub fn write(root: &Path, note: &mut Note, note_ids: &dyn Fn(&str) -> bool) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-notes-mod-rs]]
- **Calls:** [[frontmatter-parse]], [[frontmatter-render]], [[layout-folder-for]], [[layout-make-id]], [[layout-safe-stem]], [[layout-file-name]]
- **Called by:** [[engine-write-note]], [[engine-put-note]]
- **Tests:** `writes_into_type_folder_with_content_id`, `id_collision_lengthens_the_id`
