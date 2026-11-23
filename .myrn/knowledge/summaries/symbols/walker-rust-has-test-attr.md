---
id: walker-rust-has-test-attr
type: summary
status: unverified
title: Walker::rust_has_test_attr
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_has_test_attr
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::rust_has_test_attr@25a3fe11aed32b15
map_version: '2'
---
# Walker::rust_has_test_attr

Checks the attributes directly above a Rust fn for a test marker: a last path segment equal to test or rstest, or starting with test_.

## Behaviour

- #[test] and #[tokio::test] -> true
- #[test_case(1)] -> true because the name starts with test_
- #[cfg(test)] -> false; the scan keeps going through earlier attributes and comments

## Terms

test attribute, #[test], test marker, rstest, test function, attribute scan

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:731-749`
- **Signature:** `fn rust_has_test_attr(&self, n: Node<'_>) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]]
- **Called by:** [[walker-classify-rust]]
