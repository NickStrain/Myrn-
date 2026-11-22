---
id: episode-outcome
type: summary
status: unverified
title: Episode::outcome
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::outcome
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::outcome@d1097a0a7412b880
---
# Episode::outcome

Returns one of tests_passed, tests_failed, committed, edited, or no_changes, checking last_test first, then commits, then files_edited. Any last_test value other than 'pass' maps to tests_failed.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:42-50`
- **Signature:** `pub fn outcome(&self) -> &'static str`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[episode-persist]], [[log-run]], [[log-render-one]]
- **Tests:** `folds_a_session`
