---
id: frontmatter-strings
type: summary
status: unverified
title: frontmatter::strings
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::strings
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::strings@b13f67e857259868
---
# frontmatter::strings

Accepts a scalar, a sequence of scalars, or null (empty list), and returns Vec<String>; returns None if any item is not a scalar or the value is a mapping.

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:198-204`
- **Signature:** `fn strings(v: &Value) -> Option<Vec<String>>`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[frontmatter-scalar]]
- **Called by:** [[frontmatter-parse]]
