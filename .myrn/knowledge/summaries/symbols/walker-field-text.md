---
id: walker-field-text
type: summary
status: unverified
title: Walker::field_text
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::field_text
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::field_text@77d286ad0488dbca
map_version: '2'
---
# Walker::field_text

Returns the source text of a named child field such as name, path or value, or None when the field is absent.

## Behaviour

- function_item with name refresh -> Some('refresh')
- Field missing on that node kind -> None; callers then descend, skip, or fall back to an empty string
- A path field returns the whole text of the child, so a scoped name such as a::B comes back intact

## Terms

field lookup, named child, child text, field value, grammar field

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:181-183`
- **Signature:** `fn field_text(&self, n: Node<'_>, field: &str) -> Option<&'a str>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-type-name]], [[walker-rust-use-paths]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
