---
id: engine-record
type: summary
status: unverified
title: Engine::record
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::record@2a8aeb597585ed78
---
# Engine::record

Validates the item: body must be at least 12 characters, type-specific fields must match the type, evidence must cite something that exists, a summary must be about exactly one symbol or file, and fixed_in must resolve to a commit. If any check fails it returns a rejection listing every problem; otherwise it writes the root_cause note first when present, then the main note (merged into an existing bug by signature, an existing summary by target, or an existing note by id), flags contradictions with fact and decision notes on the same code, marks superseded targets, syncs, and returns the ids, updated flag and messages.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:64-371`
- **Signature:** `pub fn record(&mut self, item: RecordItem) -> Result<RecordOutcome>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[kind-as-str]], [[engine-note-roots]], [[episode-clip]], [[checks-check-type]], [[checks-check-relation]], [[engine-resolve-ref]], [[engine-check-evidence]], [[contradiction-find]], [[dedupe-find-bug]], [[dedupe-find-summary]], [[dedupe-find-id]], [[dedupe-merge]], [[recordoutcome-reject]], [[engine-put-note]], [[engine-current-session]], [[record-link]], [[symbols-resolve-all]], [[relation-parse]], [[note-new]], [[layout-make-id]], [[sync-sync]]
- **Called by:** [[engine-record-json]]
