---
id: engine-pick
type: summary
status: unverified
title: Engine::pick
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::pick
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::pick@71f3414abbf26eab
map_version: '2'
---
# Engine::pick

Turns a name into one node or a lookup result, choosing between several matches.

## Behaviour

- No hits -> NotFound; one hit -> Found
- Several hits: the first is picked only if the query is qualified and matches its qualified name exactly, or the first is a file, or the second is not exact; otherwise Ambiguous with up to 10 candidates
- A bare one-word name never picks silently between several symbols

## Terms

disambiguation, candidate choice, qualified name, path:line, ambiguous name

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:369-394`
- **Signature:** `pub(crate) fn pick(&self, query: &str) -> Result<Lookup<Node>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-find]], [[link-new]], [[seed-segments]]
- **Called by:** [[engine-explain]], [[engine-impact]], [[engine-history]], [[engine-show]], [[engine-resolve-ref]]
