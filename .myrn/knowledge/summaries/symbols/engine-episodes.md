---
id: engine-episodes
type: summary
status: unverified
title: Engine::episodes
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::episodes
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::episodes@887172043a796842
map_version: '2'
---
# Engine::episodes

Returns up to limit recorded sessions folded into episodes, newest first.

## Behaviour

- The limit applies to log files before empty ones are skipped, so fewer than limit may come back
- Empty logs are left out

## Terms

sessions list, episode history, recent sessions, session summaries

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1043-1056`
- **Signature:** `pub fn episodes(&self, limit: usize) -> Result<Vec<Episode>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-fold]], [[episodes-read]], [[episodes-list]], [[layout-episodes-dir]]
- **Called by:** [[engine-viz-sessions]], [[log-run]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
