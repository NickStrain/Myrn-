---
id: recordoutcome-reject
type: summary
status: unverified
title: RecordOutcome::reject
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::reject
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::reject@0fff94561d3fd973
---
# RecordOutcome::reject

Builds an unaccepted outcome with the given reasons as messages and empty ids, with updated set to false.

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:113-119`
- **Signature:** `pub fn reject(reasons: Vec<String>) -> RecordOutcome`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
- **Called by:** [[engine-record-json]], [[engine-record-any-json]], [[engine-record]]
