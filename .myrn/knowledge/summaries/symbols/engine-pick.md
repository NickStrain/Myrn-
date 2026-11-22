---
id: engine-pick
type: summary
status: unverified
title: Engine::pick
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::pick
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::pick@71f3414abbf26eab
---
# Engine::pick

Resolves a query to one node. Zero hits give NotFound and one gives Found; with several, the first hit is chosen only when a qualified name matches it exactly or it is a file, otherwise the result is Ambiguous with up to 10 candidates. A bare one-word name never silently picks one of several symbols.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:344-369`
- **Signature:** `pub(crate) fn pick(&self, query: &str) -> Result<Lookup<Node>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-find]], [[link-new]], [[seed-segments]]
- **Called by:** [[engine-explain]], [[engine-impact]], [[engine-history]], [[engine-show]], [[engine-resolve-ref]]
