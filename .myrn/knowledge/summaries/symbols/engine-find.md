---
id: engine-find
type: summary
status: unverified
title: Engine::find
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::find
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::find@024f9158dbf26022
---
# Engine::find

Returns graph nodes matching a query, best first, using seed::find.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:340-342`
- **Signature:** `pub fn find(&self, query: &str) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[seed-find]]
- **Called by:** [[engine-pick]]
