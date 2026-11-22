---
id: engine-show
type: summary
status: unverified
title: Engine::show
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::show
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::show@a1ef9e84ef6e8c6e
---
# Engine::show

Resolves a query to one node and returns it together with all its edges.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1020-1029`
- **Signature:** `pub fn show(&self, query: &str) -> Result<Lookup<(Node, Vec<graph::Edge>)>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[graph-edges-of]]
- **Called by:** [[show-run]]
- **Tests:** `notes_become_nodes_edges_anchors_and_summaries`
