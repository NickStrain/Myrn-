---
id: api-status
type: summary
status: unverified
title: api::Status
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Status
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Status@9f370014e4144ab5
map_version: '2'
---
# api::Status

Snapshot of the index and store: root, project id, file and symbol counts, edges, record states, notes, conflicts, connected agents and last refresh time.

## Behaviour

- files_by_language is keyed by language name, and files_with_errors counts files whose parse failed
- records_by_state lists every record status, with 0 for statuses that have no records
- last_refresh is None when no timestamp is stored or it cannot be parsed

## Terms

index stats, health summary, counts, store status, record states, connected agents

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:95-114`
- **Signature:** `pub struct Status`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
