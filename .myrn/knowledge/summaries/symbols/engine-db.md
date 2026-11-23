---
id: engine-db
type: summary
status: unverified
title: Engine::db
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::db
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::db@b362d848ac84a90d
map_version: '2'
---
# Engine::db

Gives read access to the open database handle.

## Behaviour

- Returns a shared reference to the database, with no copying

## Terms

database access, store handle, connection

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:251-253`
- **Signature:** `pub fn db(&self) -> &Db`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[main-simcheck]], [[used]], [[node]], [[has-edge]], [[note-snapshot]], [[has-edge-records]], [[node-records]], [[status]], [[status-validity]]
- **Tests:** `gate0_whole_graph_is_clean`, `forget_removes_the_record_and_what_only_it_used`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `sessions_attach_their_outcome`, `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`, `notes_become_nodes_edges_anchors_and_summaries`
