---
id: bug
type: summary
status: unverified
title: bug
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#bug
anchors:
- sym:crates/myrn-engine/tests/records.rs#bug@c0c91468715cc7cb
map_version: '2'
---
# bug

Builds the JSON payload for a bug record about unique_everseen with fixed title, body, root cause, fix and evidence, using the given error text and the first 8 characters of head as fixed_in.

## Behaviour

- error string -> copied into the error field, other fields fixed
- head -> sliced to its first 8 characters for fixed_in
- head shorter than 8 bytes -> panic on slice

## Terms

bug record, bug payload, unhashable error, fixture bug, error signature input, fix commit reference

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:134-146`
- **Signature:** `fn bug(head: &str, error: &str) -> serde_json::Value`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
