---
id: engine-db
type: summary
status: unverified
title: Engine::db
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::db
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::db@b362d848ac84a90d
---
# Engine::db

Returns a shared reference to the database handle.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:249-251`
- **Signature:** `pub fn db(&self) -> &Db`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[node]], [[has-edge]], [[note-snapshot]], [[has-edge-records]], [[node-records]]
- **Tests:** `gate0_whole_graph_is_clean`, `notes_become_nodes_edges_anchors_and_summaries`, `bugs_are_stored_deduplicated_and_recalled`, `summaries_show_on_cards_and_go_stale_with_the_code`
