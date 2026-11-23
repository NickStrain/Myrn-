---
id: file-crates-myrn-engine-src-validity-states-rs
type: summary
status: unverified
title: crates/myrn-engine/src/validity/states.rs
source: myrn summarize
about: file:crates/myrn-engine/src/validity/states.rs
anchors:
- file:crates/myrn-engine/src/validity/states.rs@533cd0ed7cb05bf8
---
# crates/myrn-engine/src/validity/states.rs

Records state transitions for the validity layer. It derives suspect and superseded states from the store's link pass, earns verified status from passing cited tests or fixing commits that were not reverted, and provides the confirm and supersede operations. Nothing is deleted. Main parts: Engine methods, the promote logic, test matching, and git history checks.

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs`
- **In:** [[dir-crates-myrn-engine-src-validity]]
- **Defines:** [[states-promotion]], [[states-suspectrecord]], [[states-changed]], [[states-confirmed]], [[engine-suspects]], [[engine-changed]], [[engine-confirm]], [[engine-supersede]], [[engine-promote]], [[engine-passing-test-runs]], [[engine-verified-by-test]], [[engine-test-needles]], [[engine-history-messages]], [[states-fix-landed]], [[states-is-shared-key]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-sync-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-validity-mod-rs]]
