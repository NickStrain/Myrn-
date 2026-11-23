---
id: engine-usage
type: summary
status: unverified
title: Engine::usage
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Engine::usage
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Engine::usage@bc2869081d953b4f
map_version: '2'
---
# Engine::usage

Returns usage history for the given node ids by delegating to usage_of on this engine's connection.

## Behaviour

- same results as usage_of for the same ids
- ids never injected in an ended session -> absent from the map
- used by myrn status and tests, per the doc comment

## Terms

usage query, status usage, stats lookup, engine usage history, injection history

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:132-134`
- **Signature:** `pub fn usage(&self, ids: &[&str]) -> Result<HashMap<String, Usage>>`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Calls:** [[learn-usage-of]]
- **Tests:** `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`
