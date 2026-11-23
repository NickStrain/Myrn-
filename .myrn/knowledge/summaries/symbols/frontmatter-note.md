---
id: frontmatter-note
type: summary
status: unverified
title: frontmatter::Note
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note@3b8c905b4507ec1d
map_version: '2'
---
# frontmatter::Note

Holds one note's header fields and body as typed data, with unrecognised header keys kept in extra.

## Behaviour

- header tags or aliases (tolerated names) -> stored in extra as written, no problem raised
- an unknown key that is not a relation name -> stored in extra and reported as a problem
- id, ntype and status are plain strings; links, anchors and evidence are separate lists

## Terms

note, header, frontmatter, front matter, typed fields, extra fields, metadata

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:28-46`
- **Signature:** `pub struct Note`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
