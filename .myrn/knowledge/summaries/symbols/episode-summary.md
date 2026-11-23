---
id: episode-summary
type: summary
status: unverified
title: Episode::summary
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::summary
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::Episode::summary@9fb46d5b8a380800
map_version: '2'
---
# Episode::summary

Builds the one-line mechanical recap of edits, reads, test runs, errors and commits, joined by commas. Symbol counts appear only when files were edited.

## Behaviour

- 1 file edited with 2 symbols -> 'edited 1 file (2 symbols)'; plural 's' is added unless the count is 1
- 3 test runs with last_test 'fail' -> '3 test runs, last failed'
- nothing recorded -> 'no changes'

## Terms

recap, one-liner, digest, status line, activity text, changelog text

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:53-85`
- **Signature:** `pub fn summary(&self) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[episode-persist]], [[outcome-commits-since]], [[git-index-commit]], [[engine-trace]], [[engine-knowledge-at]], [[knowledge-file-history]], [[engine-viz-sessions]], [[log-run]], [[log-render-one]], [[trace-run]]
- **Tests:** `folds_a_session`
