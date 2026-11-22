---
id: file-crates-myrn-engine-tests-records-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/records.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/records.rs
anchors:
- file:crates/myrn-engine/tests/records.rs@05b57669d40b20c6
---
# crates/myrn-engine/tests/records.rs

Integration tests for the record tool and distiller in myrn-engine (Step 16), covering rejection messages for invalid records, bug deduplication and recall by error signature, contradiction and supersede handling, summary staleness, session-end record requests, shell-edit capture, and batch recording. Fixtures are a temp git repo containing a small Python package, plus helpers that build episodes, consent state, and Bash events.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-records]], [[repo-records]], [[repo-58c6]], [[session]], [[has-edge-records]], [[node-records]], [[bug]], [[consent]], [[bash]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]]
