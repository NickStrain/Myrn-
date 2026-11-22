---
id: wikilink-unwrap-link
type: summary
status: unverified
title: wikilink::unwrap_link
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::unwrap_link
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::unwrap_link@73d8b6e475bcc6c3
---
# wikilink::unwrap_link

Takes a string, trims it, and if it is wrapped in `[[`/`]]` returns `normalize` of the inner text, which yields an empty string when that normalization fails. Values without both brackets are returned trimmed but otherwise unchanged, with no normalization applied.

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:58-64`
- **Signature:** `pub fn unwrap_link(value: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Calls:** [[wikilink-normalize]]
- **Called by:** [[frontmatter-parse]]
- **Tests:** `header_values`
