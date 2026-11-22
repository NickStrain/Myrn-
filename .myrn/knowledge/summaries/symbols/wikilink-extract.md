---
id: wikilink-extract
type: summary
status: unverified
title: wikilink::extract
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::extract
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::extract@257623d2ca28b5c6
---
# wikilink::extract

Takes a note body and returns the deduplicated link targets in first-appearance order, each canonicalized by normalize. Skips lines inside fenced ``` or ~~~ blocks (a fence closes only on the same marker type) and drops inline code spans by discarding odd backtick-delimited segments. Scanning is per line, so a [[ and ]] split across lines do not match, and an unclosed [[ stops scanning for that line only.

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:8-46`
- **Signature:** `pub fn extract(body: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Calls:** [[wikilink-normalize]]
- **Called by:** [[sync-link]]
- **Tests:** `obsidian_forms`, `code_is_not_linked`
