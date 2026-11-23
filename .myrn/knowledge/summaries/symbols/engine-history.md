---
id: engine-history
type: summary
status: unverified
title: Engine::history
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::history
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::history@a5ebebba0df62628
map_version: '2'
---
# Engine::history

Lists commits that changed a file, a symbol, or a directory prefix, newest first, up to limit.

## Behaviour

- File: commit_files rows whose path or old_path matches, with their change kind
- Symbol: touches edges, each marked modified
- A name matching nothing is read as a directory prefix; NotFound if no commit touched that prefix

## Terms

changelog, git log, file history, who changed, blame, commit history

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:603-676`
- **Signature:** `pub fn history(&self, query: &str, limit: usize) -> Result<Lookup<Vec<HistoryEntry>>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[id-commit-id]], [[graph-node-graph]], [[graph-neighbors]]
- **Called by:** [[history-call]], [[query-history]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
