---
id: engine-record-any-json
type: summary
status: unverified
title: Engine::record_any_json
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_any_json
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_any_json@70cb51ce245e21c4
map_version: '2'
---
# Engine::record_any_json

Accepts either one record object or an object with an items array, and records each item. A top-level session is copied into items that have none of their own.

## Behaviour

- items: [] -> one rejection saying items is empty; nothing is recorded
- items present but not an array -> one rejection: items must be a list of records
- no items key -> record_json on the whole value, returned as a one-element list

## Terms

batch, bulk record, multiple items, session default, items list

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:36-61`
- **Signature:** `pub fn record_any_json(&mut self, v: &serde_json::Value) -> Result<Vec<RecordOutcome>>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[recordoutcome-reject]], [[engine-record-json]]
- **Tests:** `one_call_records_many_items_and_trims_test_runs`
