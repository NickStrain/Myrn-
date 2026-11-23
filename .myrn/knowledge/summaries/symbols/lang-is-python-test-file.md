---
id: lang-is-python-test-file
type: summary
status: unverified
title: lang::is_python_test_file
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::is_python_test_file
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::is_python_test_file@0b89c2db1792d5cd
map_version: '2'
---
# lang::is_python_test_file

Reports whether a path looks like a Python test. It is true when the file name starts with test_, when it ends with _test.py, or when any directory segment is exactly tests or test.

## Behaviour

- tests/notes.md -> true, because the directory match ignores the extension
- src/testing.py -> false, because a segment must equal test exactly
- pkg/foo_test.py -> true

## Terms

test file, test detection, pytest, test module, test path

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:141-146`
- **Signature:** `pub fn is_python_test_file(path: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[walker-classify-python]]
