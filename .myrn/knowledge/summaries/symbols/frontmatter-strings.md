---
id: frontmatter-strings
type: summary
status: unverified
title: frontmatter::strings
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::strings
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::strings@b13f67e857259868
map_version: '2'
---
# frontmatter::strings

Reads a YAML value as a list of text items: a list of scalars, a single scalar as one item, or null as an empty list.

## Behaviour

- [a, b] -> Some with a and b
- null -> Some with an empty list
- a list containing a map -> None, so the whole field is rejected

## Terms

list, string list, multi-value, link list, targets

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:260-266`
- **Signature:** `fn strings(v: &Value) -> Option<Vec<String>>`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[frontmatter-scalar]]
- **Called by:** [[frontmatter-parse]]
