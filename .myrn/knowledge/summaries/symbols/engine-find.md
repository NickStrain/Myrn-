---
id: engine-find
type: summary
status: unverified
title: Engine::find
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::find
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::find@024f9158dbf26022
map_version: '2'
---
# Engine::find

Returns nodes matching a name, best first, by delegating to the seed search.

## Behaviour

- Returns an empty list when nothing matches
- No filtering is done here

## Terms

name search, symbol search, lookup, seed search

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:365-367`
- **Signature:** `pub fn find(&self, query: &str) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[seed-find]]
- **Called by:** [[engine-pick]], [[tools-cited]], [[ui-serve]]
