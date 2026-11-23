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

Validates record fields against the closed record-type and relation vocabularies, and resolves the references a record cites (about, located_in, evidence, relation targets) to code-graph nodes, commits or notes. Each failure comes back as a sentence the agent can act on.

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[checks-ref]], [[checks-check-type]], [[checks-check-relation]], [[checks-test-step]], [[checks-looks-like-sha]], [[engine-commit-ref]], [[engine-note-ref]], [[engine-code-ref]], [[engine-resolve-ref]], [[engine-recorded-test-run]], [[engine-check-evidence]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-model-src-id-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-distill-record-rs]]
