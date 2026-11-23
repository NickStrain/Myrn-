---
id: distill-recordoutcome
type: summary
status: unverified
title: distill::RecordOutcome
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome
anchors:
- sym:crates/myrn-engine/src/distill/mod.rs#distill::RecordOutcome@2ebb1783c0877132
map_version: '2'
---
# distill::RecordOutcome

Result of handling one record: whether it was accepted, the ids written or updated, whether an existing note was updated, and a list of messages.

## Behaviour

- Default value -> accepted false, empty ids, updated false, empty messages.
- messages holds rejection reasons or notes about what was accepted.
- ids are note ids written or updated by the engine.

## Terms

result, outcome, stored ids, update flag, rejection reasons, accepted flag

## Connections

- **Location:** `crates/myrn-engine/src/distill/mod.rs:102-110`
- **Signature:** `pub struct RecordOutcome`
- **In:** [[file-crates-myrn-engine-src-distill-mod-rs]]
