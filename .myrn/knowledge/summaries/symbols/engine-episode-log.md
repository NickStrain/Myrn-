---
id: engine-episode-log
type: summary
status: unverified
title: Engine::episode_log
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::episode_log
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::episode_log@8734f41a2713e845
---
# Engine::episode_log

Finds one session log by id, preferring an exact match and otherwise accepting a unique prefix, and returns the folded episode with its raw entries, or None.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:993-1017`
- **Signature:** `pub fn episode_log( &self, session: &str, ) -> Result<Option<(Episode, Vec<myrn_store::episodes::Entry>)>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episodes-file-name]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[log-run]]
