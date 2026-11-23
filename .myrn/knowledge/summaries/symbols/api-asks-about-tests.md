---
id: api-asks-about-tests
type: summary
status: unverified
title: api::asks_about_tests
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::asks_about_tests
anchors:
- sym:crates/myrn-engine/src/api.rs#api::asks_about_tests@ad6327387df35632
map_version: '2'
---
# api::asks_about_tests

Tells whether a task asks about tests. Sentences that mention pass are ignored, so asking to keep tests green does not count.

## Behaviour

- Which tests cover clamp gives true
- Make sure the tests still pass gives false
- Add a test for chunk gives true

## Terms

test intent, test question, tests passing, test request

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1373-1379`
- **Signature:** `fn asks_about_tests(task: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-context-excluding]]
- **Tests:** `keeping_tests_green_is_not_asking_about_tests`
