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

Declares the record schema the agent sends to the record tool, the outcome returned to it, and the closed vocabularies (RECORD_TYPES, AGENT_RELATIONS, EVIDENCE_KINDS). It also declares the submodules checks, contradiction, dedupe, mechanical, record and signature, which are not shown here, and includes a schema test.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs`
- **In:** [[dir-crates-myrn-engine-src-distill]]
- **Defines:** [[distill-recorditem]], [[distill-relationitem]], [[distill-recordoutcome]], [[recordoutcome-reject]], [[recordoutcome-render]], [[distill-one-or-many]], [[one-or-many-oneormany]]
- **Imports:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-contradiction-rs]], [[file-crates-myrn-engine-src-distill-dedupe-rs]], [[file-crates-myrn-engine-src-distill-mechanical-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-distill-signature-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-distill-checks-rs]], [[file-crates-myrn-engine-src-distill-record-rs]], [[file-crates-myrn-engine-src-lib-rs]]
