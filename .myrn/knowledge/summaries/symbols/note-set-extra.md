---
id: note-set-extra
type: summary
status: unverified
title: Note::set_extra
source: myrn summarize
about: sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::set_extra
anchors:
- sym:crates/myrn-store/src/notes/frontmatter.rs#notes::frontmatter::Note::set_extra@9e293e2c05b716a3
map_version: '2'
---
# Note::set_extra

Sets an uninterpreted header field to a text value, replacing the existing entry or appending a new one.

## Behaviour

- existing key -> value replaced in place, keeping its position in extra
- new key -> appended to the end of extra
- value is always stored as a string, even when it looks like a number

## Terms

set, replace, update, append, extra field, header field, write field

## Connections

- **Location:** `crates/myrn-store/src/notes/frontmatter.rs:109-115`
- **Signature:** `pub fn set_extra(&mut self, key: &str, value: &str)`
- **In:** [[file-crates-myrn-store-src-notes-frontmatter-rs]]
- **Called by:** [[engine-record]], [[engine-publish-plan]], [[engine-summarize]]
