---
id: walker-text
type: summary
status: unverified
title: Walker::text
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::text
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::text@322e0fc50409a021
map_version: '2'
---
# Walker::text

Returns the source text under a node, or an empty string if those bytes are not valid UTF-8.

## Behaviour

- A node covering refresh -> 'refresh'
- Invalid UTF-8 in the node span -> ''
- Multi-line nodes return the full text including newlines

## Terms

node text, source slice, node contents, raw text, token text

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:177-179`
- **Signature:** `fn text(&self, n: Node<'_>) -> &'a str`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-field-text]], [[walker-rust-macro-calls]], [[walker-note-local]], [[walker-bind-local]], [[walker-signature]], [[walker-doc]], [[walker-classify-rust]], [[walker-rust-type-name]], [[walker-rust-has-test-attr]], [[walker-rust-use-paths]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
