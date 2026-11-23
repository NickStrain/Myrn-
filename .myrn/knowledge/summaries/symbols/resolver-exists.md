---
id: resolver-exists
type: summary
status: unverified
title: Resolver::exists
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::exists
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::exists@5c4628dd28540c9f
map_version: '2'
---
# Resolver::exists

Checks whether a node with the given id exists in the code graph.

## Behaviour

- an id present in the nodes table gives true
- an unknown id gives false
- a database error is returned as an error, not false

## Terms

node check, graph lookup, id exists

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:245-247`
- **Signature:** `fn exists(&self, id: &str) -> Result<bool>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[graph-node-graph]]
- **Called by:** [[engine-init]], [[engine-open]], [[engine-records-request]], [[engine-forget]], [[engine-summarize]], [[resolver-resolve-sync]]
- **Tests:** `forget_removes_the_record_and_what_only_it_used`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `published_record_reaches_a_teammate_through_git`, `map_is_written_linked_incremental_and_searchable`
