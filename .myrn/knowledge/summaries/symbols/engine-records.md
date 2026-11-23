---
id: engine-records
type: summary
status: unverified
title: Engine::records
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::records
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::records@68fe54baf9977e43
map_version: '2'
---
# Engine::records

Lists stored records newest first by note mtime, one per id, after dropping map summaries and applying type and status filters, up to `limit`.

## Behaviour

- duplicate id across rows -> only the first (newest) row is kept
- limit 0 still returns one matching row, because the limit check runs after each push
- row whose data is not valid Note JSON -> skipped silently

## Terms

list, log, record list, filtering, deduplicate, newest first

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:109-162`
- **Signature:** `pub fn records(&self, filter: &RecordFilter, limit: usize) -> Result<Vec<RecordRow>>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[graph-node-graph]], [[note-display-title]]
- **Called by:** [[engine-viz-records]], [[log-run]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`
