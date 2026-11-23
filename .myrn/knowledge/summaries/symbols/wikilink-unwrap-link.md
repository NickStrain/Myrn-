---
id: wikilink-unwrap-link
type: summary
status: unverified
title: wikilink::unwrap_link
source: myrn summarize
about: sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::unwrap_link
anchors:
- sym:crates/myrn-store/src/notes/wikilink.rs#notes::wikilink::unwrap_link@73d8b6e475bcc6c3
map_version: '2'
---
# wikilink::unwrap_link

Trims the value and, if it is wrapped in [[ ]], returns the inner text normalized to a bare note name, or an empty string if normalize yields nothing. A value that is not wrapped is returned trimmed and otherwise unchanged.

## Behaviour

- '[[rootcause-a12b08]]' -> 'rootcause-a12b08'; ' [[a|b]] ' -> 'a'
- '[[ ]]' or '[[]]' -> '' (empty string, not None)
- 'auth::refresh' -> 'auth::refresh' unchanged; '[[a]' has only one side wrapped, so it is returned as '[[a]' with brackets kept

## Terms

header value, strip brackets, unwrap link, bracketed value, wrapped link, link header

## Connections

- **Location:** `crates/myrn-store/src/notes/wikilink.rs:58-64`
- **Signature:** `pub fn unwrap_link(value: &str) -> String`
- **In:** [[file-crates-myrn-store-src-notes-wikilink-rs]]
- **Calls:** [[wikilink-normalize]]
- **Called by:** [[frontmatter-parse]]
- **Tests:** `header_values`
