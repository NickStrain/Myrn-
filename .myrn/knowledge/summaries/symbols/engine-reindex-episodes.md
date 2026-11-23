---
id: engine-reindex-episodes
type: summary
status: unverified
title: Engine::reindex_episodes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::reindex_episodes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::reindex_episodes@7301e53dbb8090cd
map_version: '2'
---
# Engine::reindex_episodes

Rebuilds the episode nodes for every stored session log and returns how many were written.

## Behaviour

- Empty logs are skipped and not counted
- Called by rebuild after the index is recreated

## Terms

episode rebuild, session nodes, after rebuild, episode index

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1029-1040`
- **Signature:** `pub fn reindex_episodes(&self) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episode-persist]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-rebuild]], [[engine-gc]], [[session-history]], [[session]]
