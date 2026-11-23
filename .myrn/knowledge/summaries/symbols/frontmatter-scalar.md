---
id: frontmatter-scalar
type: summary
status: unverified
title: frontmatter::scalar
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::scalar
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::scalar@b182c47f3f43d0d6
map_version: '2'
---
# frontmatter::scalar

Converts a YAML string, number or boolean to text, and returns None for null, lists and maps.

## Behaviour

- number 0.5 -> '0.5'; boolean true -> 'true'
- list or map -> None
- null -> None

## Terms

scalar, text value, stringify, coerce, plain value

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:250-257`
- **Signature:** `fn scalar(v: &Value) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[note-extra-str]], [[frontmatter-strings]], [[frontmatter-parse]]
