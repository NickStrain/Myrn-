---
id: episode-fold
type: summary
status: unverified
title: episode::fold
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::fold
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::fold@8524d2e4c076d9ae
map_version: '2'
---
# episode::fold

Folds a session's ordered log entries into one Episode, tallying prompts, file reads and edits, tests, commands, errors and commits.

## Behaviour

- first entry sets started and agent; first prompt text becomes task, later prompts only increment the count; empty input gives started 0 and empty agent
- tool entry with outcome 'file_edit' adds its files to edited, otherwise to read; a file both read and edited ends up only in edited
- keeps only the last 50 commands and last 20 errors; session_end sets ended and adds only commits not already present

## Terms

replay, aggregate, rollup, reduce, session log, tally, build episode

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:89-150`
- **Signature:** `pub fn fold(session: &str, entries: &[Entry]) -> Episode`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]]
- **Tests:** `folds_a_session`
