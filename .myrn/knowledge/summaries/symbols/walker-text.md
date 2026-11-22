---
id: walker-text
type: summary
status: unverified
title: Walker::text
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::text
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::text@322e0fc50409a021
---
# Walker::text

Returns the UTF-8 source text of a node, or empty string if invalid UTF-8.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:177-179`
- **Signature:** `fn text(&self, n: Node<'_>) -> &'a str`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-field-text]], [[walker-rust-macro-calls]], [[walker-note-local]], [[walker-bind-local]], [[walker-signature]], [[walker-doc]], [[walker-classify-rust]], [[walker-rust-type-name]], [[walker-rust-has-test-attr]], [[walker-rust-use-paths]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
