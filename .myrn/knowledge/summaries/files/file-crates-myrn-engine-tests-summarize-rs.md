---
id: file-crates-myrn-engine-tests-summarize-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/summarize.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/summarize.rs
anchors:
- file:crates/myrn-engine/tests/summarize.rs@7d0fff00285986e4
---
# crates/myrn-engine/tests/summarize.rs

Integration test for the summarize feature: it checks that summary notes are written and linked, that only changed code is re-summarized, and that search finds code by its summary. It also holds the shared helpers: a fixture file writer, a temporary git repo builder, a stub model command that logs each call, and a call counter.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-summarize]], [[repo-summarize]], [[model]], [[calls]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-summarize-rs]]
