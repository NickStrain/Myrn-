---
id: bug
type: summary
status: unverified
title: bug
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#bug
anchors:
- sym:crates/myrn-engine/tests/records.rs#bug@c0c91468715cc7cb
---
# bug

Builds a bug record JSON for unique_everseen with the given error text, using the first 8 chars of head as fixed_in and evidence citing test_everseen and a pytest test_run.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:134-146`
- **Signature:** `fn bug(head: &str, error: &str) -> serde_json::Value`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`
