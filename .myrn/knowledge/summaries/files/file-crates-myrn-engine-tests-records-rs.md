---
id: file-crates-myrn-engine-tests-records-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/records.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/records.rs
anchors:
- file:crates/myrn-engine/tests/records.rs@61b59c665dcfb4fa
---
# crates/myrn-engine/tests/records.rs

Integration tests for the record tool, distiller checks and recall, run against a temporary git repo with a small Python package. Fixture sources (PY) and helpers (write, repo, session, consent, bash, has_edge, node, bug) sit above tests covering rejections, bug dedup and recall, contradictions and supersession, summary staleness, session-end record requests, and shell-edit capture.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-records]], [[repo-records]], [[repo-58c6]], [[session]], [[has-edge-records]], [[node-records]], [[bug]], [[consent]], [[bash]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-event-rs]], [[file-crates-myrn-store-src-episodes-rs]], [[file-crates-myrn-store-src-graph-rs]]
