---
id: frontmatter-scalar
type: summary
status: unverified
title: frontmatter::scalar
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::scalar
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::scalar@b182c47f3f43d0d6
---
# frontmatter::scalar

Converts a YAML string, number or bool to a String; returns None for null, sequences and mappings.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:188-195`
- **Signature:** `fn scalar(v: &Value) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[frontmatter-strings]], [[frontmatter-parse]]
