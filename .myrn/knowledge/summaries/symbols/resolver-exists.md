---
id: resolver-exists
type: summary
status: unverified
title: Resolver::exists
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::exists
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::exists@5c4628dd28540c9f
---
# Resolver::exists

Returns whether a node with the given id exists in the graph.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:235-237`
- **Signature:** `fn exists(&self, id: &str) -> Result<bool>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[graph-node-graph]]
- **Called by:** [[engine-init]], [[engine-open]], [[engine-records-request]], [[engine-summarize]], [[resolver-resolve-sync]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
