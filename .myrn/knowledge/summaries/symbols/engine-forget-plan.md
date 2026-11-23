---
id: engine-forget-plan
type: summary
status: unverified
title: Engine::forget_plan
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::forget_plan
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::forget_plan@2f0da21469cddf2a
map_version: '2'
---
# Engine::forget_plan

Computes what `forget` would remove for an id without changing anything: the record, root causes only it cites, its note files, and other records linking to it.

## Behaviour

- root cause added only when no other note links to it through note evidence and its graph node exists
- unlinked_from lists other records that link to any planned id, excluding those already planned
- unknown id -> error from record_row

## Terms

dry run, preview, plan, root cause, caused_by, impact

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:397-439`
- **Signature:** `pub fn forget_plan(&self, id: &str) -> Result<Forgotten>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[engine-note-file]], [[engine-record-row]], [[graph-node-graph]]
- **Called by:** [[engine-forget]], [[forget-run]]
- **Tests:** `forget_removes_the_record_and_what_only_it_used`
