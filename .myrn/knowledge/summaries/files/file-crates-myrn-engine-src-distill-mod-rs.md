---
id: file-crates-myrn-engine-src-distill-mod-rs
type: summary
status: unverified
title: crates/myrn-engine/src/distill/mod.rs
source: myrn summarize
about: file:crates/myrn-engine/src/distill/mod.rs
anchors:
- file:crates/myrn-engine/src/distill/mod.rs@d8ee8246cb2f3447
---
# crates/myrn-engine/src/distill/mod.rs

Root of the distillation module. It declares the submodules (checks, contradiction, dedupe, mechanical, record, signature) and defines the closed RECORD_TYPES, AGENT_RELATIONS and EVIDENCE_KINDS lists, plus the RecordItem, RelationItem and RecordOutcome types used by the `record` tool. It also holds the one_or_many deserializer and a schema unit test.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[distill-recorditem]], [[distill-relationitem]], [[distill-recordoutcome]], [[recordoutcome-reject]], [[recordoutcome-render]], [[distill-one-or-many]], [[one-or-many-oneormany]]
- **Imports:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-validity-states-rs]]
