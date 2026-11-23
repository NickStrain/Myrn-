---
id: file-crates-myrn-engine-tests-history-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/history.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/history.rs
anchors:
- file:crates/myrn-engine/tests/history.rs@f34be695d1b4d397
---
# crates/myrn-engine/tests/history.rs

Integration tests for the history features (record listing, trace, knowledge diff, forget, gc) run against a temporary git repo with a small Python package. It holds four helpers that write fixture files, commit them with git2, build an Engine, and append synthetic agent episodes.

## Connections

- **Location:** `crates/myrn-engine/tests/history.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-history]], [[commit-all-history]], [[setup-history]], [[session-history]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-store-src-episodes-rs]]
