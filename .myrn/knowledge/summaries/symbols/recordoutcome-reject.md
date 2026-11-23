---
id: recordoutcome-reject
type: summary
status: unverified
title: RecordOutcome::reject
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::reject
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome::reject@0fff94561d3fd973
map_version: '2'
---
# RecordOutcome::reject

Builds a rejected outcome from a list of reasons, with nothing stored.

## Behaviour

- reject(vec!["x"]) -> accepted false, messages ['x'], ids empty, updated false.
- reject(vec![]) -> accepted false with no messages; no error is raised.
- Does not touch the store; it only builds the value.

## Terms

rejection, refusal, failed check, validation failure, reject outcome

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:113-119`
- **Signature:** `pub fn reject(reasons: Vec<String>) -> RecordOutcome`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
- **Called by:** [[engine-record-json]], [[engine-record-any-json]], [[engine-record]]
