---
id: note-new
type: summary
status: unverified
title: Note::new
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::new
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::new@9b188b96b31abf4e
---
# Note::new

Builds a note with an empty id (notes::write assigns it later), the given type and body, status 'unverified', and empty links, anchors, evidence and extra.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:95-110`
- **Signature:** `pub fn new(ntype: &str, body: &str) -> Note`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[status-as-str]]
- **Called by:** [[engine-record]], [[frontmatter-parse]]
- **Tests:** `merge_combines_and_refreshes_anchors`, `write_note_writes_first_then_indexes_and_shared_wins`, `writes_into_type_folder_with_content_id`, `id_collision_lengthens_the_id`
