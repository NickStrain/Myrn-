---
id: note-new
type: summary
status: unverified
title: Note::new
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::new
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::new@9b188b96b31abf4e
map_version: '2'
---
# Note::new

Builds a note with the given type and body, an empty id and status unverified, and no title, links, anchors or evidence.

## Behaviour

- new('bug', 'text') -> id empty, ntype bug, status unverified, body text
- id stays empty until the note writer assigns one from the content
- confidence, source, signature and title are all None

## Terms

new note, create, blank note, default note, unverified, constructor

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:118-133`
- **Signature:** `pub fn new(ntype: &str, body: &str) -> Note`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[status-as-str]]
- **Called by:** [[engine-record]], [[frontmatter-parse]]
- **Tests:** `merge_combines_and_refreshes_anchors`, `clean_strips_body_title_and_evidence`, `write_note_writes_first_then_indexes_and_shared_wins`, `writes_into_type_folder_with_content_id`, `id_collision_lengthens_the_id`
