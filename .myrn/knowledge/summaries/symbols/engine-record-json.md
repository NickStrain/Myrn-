---
id: engine-record-json
type: summary
status: unverified
title: Engine::record_json
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_json
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_json@a088d8456a6ec1ec
map_version: '2'
---
# Engine::record_json

Deserializes a JSON value into a RecordItem and passes it to record. A schema mismatch becomes a rejection outcome instead of an error.

## Behaviour

- valid item JSON -> the outcome returned by record
- wrong shape or field types -> rejection with one problem starting 'the record does not match the schema'
- errors from record itself (store or lookup failures) propagate as Err

## Terms

json parsing, schema validation, deserialize, raw json entry

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:24-31`
- **Signature:** `pub fn record_json(&mut self, v: &serde_json::Value) -> Result<RecordOutcome>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[recordoutcome-reject]], [[engine-record]]
- **Called by:** [[engine-record-any-json]], [[engine-supersede]], [[fact]], [[fact-validity]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`, `forget_removes_the_record_and_what_only_it_used`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `rejections_say_what_to_fix`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`
