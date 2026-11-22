---
id: walker-rust-has-test-attr
type: summary
status: unverified
title: Walker::rust_has_test_attr
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_has_test_attr
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_has_test_attr@25a3fe11aed32b15
---
# Walker::rust_has_test_attr

Looks back through preceding attributes and comments for a `test`, `test_*`, or `rstest` attribute; this is what sets is_test on Rust functions.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:731-749`
- **Signature:** `fn rust_has_test_attr(&self, n: Node<'_>) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]]
- **Called by:** [[walker-classify-rust]]
