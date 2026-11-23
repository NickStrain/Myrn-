---
id: note-extra-str
type: summary
status: unverified
title: Note::extra_str
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::extra_str
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::extra_str@6330f388367576c0
map_version: '2'
---
# Note::extra_str

Returns an uninterpreted header field as trimmed text, or None when it is absent, not a scalar, or blank.

## Behaviour

- key holds the number 1.5 -> Some('1.5')
- key holds a list or a map -> None
- key holds only whitespace -> None

## Terms

extra field, header field, lookup, get, string value, uninterpreted field

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:99-106`
- **Signature:** `pub fn extra_str(&self, key: &str) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Calls:** [[frontmatter-scalar]]
- **Called by:** [[engine-publish-plan]], [[engine-summarize]], [[sync-link]]
