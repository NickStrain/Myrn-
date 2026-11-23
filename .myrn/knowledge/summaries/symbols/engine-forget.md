---
id: engine-forget
type: summary
status: unverified
title: Engine::forget
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::forget
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::forget@8945d3b644357a0e
map_version: '2'
---
# Engine::forget

Permanently deletes an id and its planned root causes: removes links to them from other notes, deletes every note file copy, clears usage rows, then resyncs the index.

## Behaviour

- other notes are rewritten only when one of their links was actually removed
- missing files are skipped; a failing delete returns an error with earlier steps already applied (no rollback)
- returns the Forgotten computed by forget_plan at the start

## Terms

delete, remove permanently, purge, erase, unlink, delete note files

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:443-470`
- **Signature:** `pub fn forget(&mut self, id: &str) -> Result<Forgotten>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[engine-note-roots]], [[dedupe-find-id]], [[engine-put-note]], [[engine-forget-plan]], [[sync-sync]], [[resolver-exists]]
- **Called by:** [[forget-run]]
- **Tests:** `forget_removes_the_record_and_what_only_it_used`, `shared_records_that_disagree_are_flagged_for_a_person`
