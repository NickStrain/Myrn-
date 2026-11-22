---
id: lang-is-python-test-file
type: summary
status: unverified
title: lang::is_python_test_file
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::is_python_test_file
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::is_python_test_file@0b89c2db1792d5cd
---
# lang::is_python_test_file

Returns true if the basename starts with test_ or ends with _test.py, or if any path segment is exactly tests or test. The path does not need to end in .py for the prefix check, and any matching directory segment counts.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:141-146`
- **Signature:** `pub fn is_python_test_file(path: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[walker-classify-python]]
