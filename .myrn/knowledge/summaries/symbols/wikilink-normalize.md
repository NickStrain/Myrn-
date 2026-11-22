---
id: wikilink-normalize
type: summary
status: unverified
title: wikilink::normalize
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::normalize
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::normalize@0cae3e1cd86326fc
---
# wikilink::normalize

Takes the raw text between [[ and ]] and returns Some(basename) or None. It drops any |alias and #heading suffix, trims whitespace, strips a trailing .md (repeatedly), keeps only the last path segment after /, and returns None if nothing remains, so [[ ]] and [[#x]] yield None. Case is preserved, and folder components are discarded, so same-named notes in different folders collapse to one target.

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:49-55`
- **Signature:** `pub fn normalize(raw: &str) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Called by:** [[wikilink-extract]], [[wikilink-unwrap-link]]
