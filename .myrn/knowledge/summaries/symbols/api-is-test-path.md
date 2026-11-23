---
id: api-is-test-path
type: summary
status: unverified
title: api::is_test_path
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::is_test_path
anchors:
- sym:crates/myrn-engine/src/api.rs#api::is_test_path@442c3e2bcf19c00f
map_version: '2'
---
# api::is_test_path

Tells whether a path looks like a test file or sits in a test directory.

## Behaviour

- A directory named test, tests, __tests__ or spec gives true
- A file name starting with test_, or containing _test., .test. or .spec. gives true
- A plain source path gives false

## Terms

test file, test detection, spec file, test directory

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1360-1368`
- **Signature:** `fn is_test_path(path: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
