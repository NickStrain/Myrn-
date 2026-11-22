---
id: walker-field-text
type: summary
status: unverified
title: Walker::field_text
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::field_text
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::field_text@77d286ad0488dbca
---
# Walker::field_text

Returns the text of a named field child, or None when the field is absent.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:181-183`
- **Signature:** `fn field_text(&self, n: Node<'_>, field: &str) -> Option<&'a str>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]]
- **Called by:** [[walker-classify-rust]], [[walker-rust-type-name]], [[walker-rust-use-paths]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
