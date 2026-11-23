---
id: wikilink-extract
type: summary
status: unverified
title: wikilink::extract
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::extract
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::extract@257623d2ca28b5c6
map_version: '2'
---
# wikilink::extract

Scans a note body line by line and returns the deduplicated [[link]] targets in first-seen order, skipping fenced code blocks and inline code spans. Each target is passed through normalize.

## Behaviour

- '[[a]] [[b]] [[a]]' -> ['a', 'b']; duplicates are dropped, first appearance kept
- lines inside ``` or ~~~ fences and the fence lines themselves are skipped; a ~~~ line does not close a ``` fence
- '`[[c]]`' is skipped; a single unpaired backtick drops the rest of that line; '[[ ]]' yields nothing; '[[' with no ']]' on the line yields nothing

## Terms

wikilink, obsidian link, backlink, link target, note reference, embed, code fence, related edge

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:8-46`
- **Signature:** `pub fn extract(body: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Calls:** [[wikilink-normalize]]
- **Called by:** [[engine-viz-notes-graph]], [[sync-link]]
- **Tests:** `obsidian_forms`, `code_is_not_linked`
