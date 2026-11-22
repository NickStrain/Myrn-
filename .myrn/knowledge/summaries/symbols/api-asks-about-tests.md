---
id: api-asks-about-tests
type: summary
status: unverified
title: api::asks_about_tests
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::asks_about_tests
anchors:
- sym:crates/myrn-engine/src/api.rs#api::asks_about_tests@c0ab5829c7d2a3fe
---
# api::asks_about_tests

Returns true when any word in the task starts with test or equals spec or specs.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1299-1303`
- **Signature:** `fn asks_about_tests(task: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-context-excluding]]
