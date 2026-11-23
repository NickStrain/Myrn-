---
id: file-crates-myrn-engine-tests-packing-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/packing.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/packing.rs
anchors:
- file:crates/myrn-engine/tests/packing.rs@90e1e4909b79e2ed
---
# crates/myrn-engine/tests/packing.rs

Integration tests for the myrn-engine context packs: re-exported namespace calls linking to tests, pack size and content, and noise filtering. It also holds two helpers, write (lays files out on disk) and repo (builds a temp git repo with one commit and an Engine over it).

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-packing]], [[repo-packing]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]]
