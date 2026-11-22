---
id: file-crates-myrn-engine-src-capture-outcome-rs
type: summary
status: unverified
title: crates/myrn-engine/src/capture/outcome.rs
source: myrn summarize
about: file:crates/myrn-engine/src/capture/outcome.rs
anchors:
- file:crates/myrn-engine/src/capture/outcome.rs@2edbc10ceb49f5ad
---
# crates/myrn-engine/src/capture/outcome.rs

Outcome-detection helpers for capture: a table-driven matcher that recognises test-runner invocations in shell commands, and a git history walker that lists recent commits. It also holds a unit test for the matcher that covers both positive and negative command examples.

## Connections

- **Location:** `crates/myrn-engine/src/capture/outcome.rs`
- **In:** [[dir-crates-myrn-engine-src-capture]]
- **Defines:** [[outcome-is-test-command]], [[outcome-commits-since]]
- **Imported by:** [[file-crates-myrn-engine-src-capture-mod-rs]]
