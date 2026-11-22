---
id: walker-classify-python
type: summary
status: unverified
title: Walker::classify_python
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_python
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_python@38d6aee596899b5e
---
# Walker::classify_python

Maps Python defs and classes, call refs (identifier or attribute), and import and from-import statements to records; a `test*` function is a test only in test files or in a class whose name starts with `Test`.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:805-915`
- **Signature:** `fn classify_python<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-is-python-test-file]], [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-in-type-scope]], [[parse-truncate]]
- **Called by:** [[walker-classify]]
