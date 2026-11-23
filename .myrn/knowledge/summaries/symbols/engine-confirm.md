---
id: engine-confirm
type: summary
status: unverified
title: Engine::confirm
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::confirm
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::confirm@b33c573a27d8c331
map_version: '2'
---
# Engine::confirm

Rehashes a record's anchors to the current body hashes, rewrites the note, and resyncs so a suspect record returns to its prior status.

## Behaviour

- Unknown id, or a summary-type record -> error, nothing written
- Any anchor with no current body hash -> error listing those targets, nothing written
- Stored hash length is kept, clamped to 6-16 characters, default 16 when absent

## Terms

accept, reconfirm, re-hash anchors, clear suspect, resync

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:108-161`
- **Signature:** `pub fn confirm(&mut self, id: &str) -> Result<Confirmed>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[engine-note-roots]], [[dedupe-find-id]], [[engine-put-note]], [[graph-node-graph]], [[sync-sync]]
- **Called by:** [[verify-run]]
- **Tests:** `confirm_moves_anchors_to_the_current_code`
