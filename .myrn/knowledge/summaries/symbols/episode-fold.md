---
id: episode-fold
type: summary
status: unverified
title: episode::fold
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::fold
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::fold@8524d2e4c076d9ae
---
# episode::fold

Folds a session's entries into an Episode: the first entry sets started and agent, the first prompt sets task, 'tool' entries with outcome 'file_edit' add to edits and others to reads, and each 'session_end' sets ended and appends new commits. Afterward, files that were also edited are removed from files_read, and commands and errors are trimmed to the newest 50 and 20.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:89-150`
- **Signature:** `pub fn fold(session: &str, entries: &[Entry]) -> Episode`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]]
- **Tests:** `folds_a_session`
