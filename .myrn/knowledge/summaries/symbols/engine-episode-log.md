---
id: engine-episode-log
type: summary
status: unverified
title: Engine::episode_log
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::episode_log
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::episode_log@8734f41a2713e845
map_version: '2'
---
# Engine::episode_log

Finds one session's log by id or unique prefix and returns the folded episode with its entries.

## Behaviour

- An exact id match wins over prefix matches
- A prefix matching several sessions with no exact match -> None; an unknown id -> None

## Terms

session log, session lookup, prefix match, transcript, episode entries

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1059-1083`
- **Signature:** `pub fn episode_log( &self, session: &str, ) -> Result<Option<(Episode, Vec<myrn_store::episodes::Entry>)>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episodes-file-name]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-trace]], [[engine-attach-outcome]], [[log-run]], [[ui-serve]]
