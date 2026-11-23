---
id: walker-classify-python
type: summary
status: unverified
title: Walker::classify_python
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_python
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::classify_python@38d6aee596899b5e
map_version: '2'
---
# Walker::classify_python

Maps Python def and class nodes to defs, calls to references, and import statements to Import records. Import nodes return Skip after recording, and comments and strings are skipped.

## Behaviour

- In a file named test_session.py, a module-level def test_refresh is is_test; the same name in app.py is not
- A method in class TestAuth is is_test only if its own name starts with test
- import os.path gives spec 'os.path' alias 'os.path'; from .m import a as b gives spec '.m.a' alias 'b'

## Terms

Python classification, pytest test, class method, import statement, from import, module import

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:805-915`
- **Signature:** `fn classify_python<'t>(&mut self, n: Node<'t>) -> Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[lang-is-python-test-file]], [[walker-text]], [[walker-field-text]], [[walker-line]], [[walker-push-import]], [[walker-in-type-scope]], [[parse-truncate]]
- **Called by:** [[walker-classify]]
