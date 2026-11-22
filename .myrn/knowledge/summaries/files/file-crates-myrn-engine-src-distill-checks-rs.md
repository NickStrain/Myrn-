---
id: file-crates-myrn-engine-src-distill-checks-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/checks.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/checks.rs
anchors:
- file:crates/myrn-engine/src/distill/checks.rs@9f66130a0d92ee41
---
# crates/myrn-engine/src/distill/checks.rs

Validates records written by an agent: checks the record type and relation against closed vocabularies, parses test commands from shell lines, and resolves each reference (note id, commit sha, symbol, test, file) against the code graph and notes, returning either a resolved Ref or a message the agent can act on. The Engine methods at the bottom do the resolution and evidence checking.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[checks-ref]], [[checks-check-type]], [[checks-check-relation]], [[checks-test-step]], [[checks-looks-like-sha]], [[engine-commit-ref]], [[engine-note-ref]], [[engine-code-ref]], [[engine-resolve-ref]], [[engine-recorded-test-run]], [[engine-check-evidence]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]]
