---
id: directives-stripped
type: summary
status: unverified
title: directives::Stripped
source: myrn summarize
about: sym:crates/myrn-store/src/notes/directives.rs#notes::directives::Stripped
anchors:
- sym:crates/myrn-store/src/notes/directives.rs#notes::directives::Stripped@5b3b7f8a37f080f6
map_version: '2'
---
# directives::Stripped

Result of strip: the cleaned text plus the list of removed pieces. Its default value is empty text and no removed entries.

## Behaviour

- Default value has empty text and an empty removed list.
- removed can hold whole non-directive line text when a line is dropped, not only directive sentences.
- Each removed entry is trimmed of surrounding whitespace.

## Terms

cleaned text, removed pieces, sanitized result, stripped note, dropped sentences, filter result

## Connections

- **Location:** `crates/myrn-store/src/notes/directives.rs:19-22`
- **Signature:** `pub struct Stripped`
- **In:** [[file-crates-myrn-store-src-notes-directives-rs]]
