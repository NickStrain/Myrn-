---
id: engine-record-any-json
type: summary
status: unverified
title: Engine::record_any_json
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_any_json
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_any_json@70cb51ce245e21c4
---
# Engine::record_any_json

Accepts either one record or an object with an items array and returns one RecordOutcome per item, in order. Items without their own session field inherit the top-level session; an empty items array or a non-array items value is rejected, and each item is checked independently.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:36-61`
- **Signature:** `pub fn record_any_json(&mut self, v: &serde_json::Value) -> Result<Vec<RecordOutcome>>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[recordoutcome-reject]], [[engine-record-json]]
- **Tests:** `one_call_records_many_items_and_trims_test_runs`
