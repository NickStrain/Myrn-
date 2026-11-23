---
id: frontmatter-parse
type: summary
status: unverified
title: frontmatter::parse
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::parse
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::parse@a92f294521456283
map_version: '2'
---
# frontmatter::parse

Parses a whole note into a Note plus problems, validating each header field and using the file stem and folder as fallbacks for id and type.

## Behaviour

- header present but no id -> id becomes the file stem with problem 'no id; using the file name'; no header -> stem used silently
- id that does not match the file name (compared lowercased) -> problem reported, but the header id is kept
- unknown type -> problem, type set to the folder's type or 'fact'; unknown status -> problem, status set to 'unverified'

## Terms

parse, validation, fallback id, defaults, malformed header, problems, indexing

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:270-444`
- **Signature:** `pub fn parse(text: &str, stem: &str, folder: Option<&str>) -> Parsed`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[status-as-str]], [[status-parse]], [[relation-parse]], [[note-new]], [[frontmatter-split]], [[frontmatter-scalar]], [[frontmatter-strings]], [[layout-safe-stem]], [[wikilink-unwrap-link]]
- **Called by:** [[knowledge-parse-note]], [[engine-publish]], [[engine-gc]], [[notes-write]], [[sync-sync]]
- **Tests:** `parses_the_design_example`, `round_trips`, `round_trips_lists_extra_and_crlf`, `malformed_headers_are_flagged_not_dropped`, `writes_into_type_folder_with_content_id`
