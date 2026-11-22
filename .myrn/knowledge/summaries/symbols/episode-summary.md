---
id: episode-summary
type: summary
status: unverified
title: Episode::summary
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::summary
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::summary@9fb46d5b8a380800
---
# Episode::summary

Builds a comma-separated line such as 'edited 2 files (3 symbols), read 1 file, 1 test run, last failed', or 'no changes' if empty. Test-run count is passed plus failed, and the last result is reported as 'last passed' or 'last failed'.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:53-85`
- **Signature:** `pub fn summary(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[episode-persist]], [[outcome-commits-since]], [[git-index-commit]], [[log-run]], [[log-render-one]]
- **Tests:** `folds_a_session`
