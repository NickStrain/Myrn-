---
id: frontmatter-link
type: summary
status: unverified
title: frontmatter::Link
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Link
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Link@38d05bd774c2c555
map_version: '2'
---
# frontmatter::Link

One typed relation from a header field pointing at a target note or commit, with an inverse flag for the fixed_in spelling.

## Behaviour

- caused_by: '[[rootcause-a12b08]]' -> rel CausedBy, target rootcause-a12b08, inverse false
- fixed_in: 4be2a07 -> rel Fixes, target 4be2a07, inverse true, meaning the commit fixes this note
- inverse defaults to false when it is missing from serialised data

## Terms

relation, link, edge, reference, inverse link, fixes, typed link

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:51-56`
- **Signature:** `pub struct Link`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
