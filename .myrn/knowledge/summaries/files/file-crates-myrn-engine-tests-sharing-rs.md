---
id: file-crates-myrn-engine-tests-sharing-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/sharing.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/sharing.rs
anchors:
- file:crates/myrn-engine/tests/sharing.rs@5fe2705095a6ab7d
---
# crates/myrn-engine/tests/sharing.rs

Integration tests for Git-based sharing of knowledge records (Step 21): publishing a record into the repo, importing teammates' records via git pull, sanitising injected directives, flagging contradictory shared facts, and local verification. Helpers build a sample Python package, two clones with separate engine homes, and commit/pull utilities.

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[files]], [[write-sharing]], [[commit-all-sharing]], [[pull]], [[dev]], [[two-clones]], [[fact]], [[status]], [[pull-keeping-local]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]]
