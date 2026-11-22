---
id: engine-reindex-episodes
type: summary
status: unverified
title: Engine::reindex_episodes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::reindex_episodes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::reindex_episodes@7301e53dbb8090cd
---
# Engine::reindex_episodes

Rebuilds episode nodes from every non-empty episode log and returns how many were written.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:963-974`
- **Signature:** `pub fn reindex_episodes(&self) -> Result<usize>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episode-persist]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-rebuild]], [[session]]
