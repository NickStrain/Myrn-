---
id: api-is-test-path
type: summary
status: unverified
title: api::is_test_path
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::is_test_path
anchors:
- sym:crates/myrn-engine/src/api.rs#api::is_test_path@442c3e2bcf19c00f
---
# api::is_test_path

Returns true for paths under test, tests, __tests__ or spec directories and for files named test_*, *_test.*, *.test.* or *.spec.*.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1288-1296`
- **Signature:** `fn is_test_path(path: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
