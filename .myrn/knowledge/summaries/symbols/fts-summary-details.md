---
id: fts-summary-details
type: summary
status: unverified
title: fts::summary_details
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::summary_details
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::summary_details@23760373fc76e7b4
map_version: '2'
---
# fts::summary_details

Returns the behaviour text and terms text of one code-map note joined by a space, lowercased. It returns an empty string when the id has no row.

## Behaviour

- id with behaviour `Returns the token.` and terms `Token Lookup` -> `returns the token. token lookup`
- id not present in summary_search -> empty string
- behaviour or terms is NULL -> empty string, because SQLite `||` yields NULL and unwrap_or_default maps it to empty

## Terms

summary text, note text, behaviour text, code map note, lowercase text, details lookup, note body, fetch summary

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:112-121`
- **Signature:** `pub fn summary_details(conn: &Connection, id: &str) -> Result<String>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[seed-seeds]]
