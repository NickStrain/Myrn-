---
id: dir-crates-myrn-engine-tests
type: summary
status: unverified
title: crates/myrn-engine/tests/
source: myrn summarize
about: module:crates/myrn-engine/tests
anchors:
- module:crates/myrn-engine/tests@a4b982237bb8e970
---
# crates/myrn-engine/tests/

Acceptance and integration tests for the myrn-engine crate, exercised as Rust integration tests in one file per feature area. gate0.rs runs the full tool surface (explain, impact, context, brief, refresh, history, rebuild) against the multi-language fixture at tests/fixtures/repos/gate0 and asserts exact call and test edges; notes.rs, packing.rs, records.rs and summarize.rs cover markdown note indexing, context packs and lookup, the record/distiller tool, and the summarize pipeline, mostly building temp git repos with inline or copied fixtures.

## Connections

- **In:** [[dir-crates-myrn-engine]]
- **Files:** [[file-crates-myrn-engine-tests-gate0-rs]], [[file-crates-myrn-engine-tests-notes-rs]], [[file-crates-myrn-engine-tests-packing-rs]], [[file-crates-myrn-engine-tests-records-rs]], [[file-crates-myrn-engine-tests-summarize-rs]]
