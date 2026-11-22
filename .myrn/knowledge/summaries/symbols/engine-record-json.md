---
id: engine-record-json
type: summary
status: unverified
title: Engine::record_json
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_json
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record_json@a088d8456a6ec1ec
---
# Engine::record_json

Deserializes a JSON value into a RecordItem and calls record; a schema mismatch returns a rejected outcome containing the serde error instead of an Err.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:24-31`
- **Signature:** `pub fn record_json(&mut self, v: &serde_json::Value) -> Result<RecordOutcome>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[recordoutcome-reject]], [[engine-record]]
- **Called by:** [[engine-record-any-json]]
- **Tests:** `rejections_say_what_to_fix`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`, `summaries_show_on_cards_and_go_stale_with_the_code`
