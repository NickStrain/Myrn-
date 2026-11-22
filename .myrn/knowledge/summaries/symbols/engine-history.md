---
id: engine-history
type: summary
status: unverified
title: Engine::history
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::history
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::history@a5ebebba0df62628
---
# Engine::history

Lists commits for a file, a symbol or a directory prefix, newest first and cut to limit. Commits missing from the graph are skipped, and a prefix with no commits returns NotFound.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:574-647`
- **Signature:** `pub fn history(&self, query: &str, limit: usize) -> Result<Lookup<Vec<HistoryEntry>>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[id-commit-id]], [[graph-node-graph]], [[graph-neighbors]]
- **Called by:** [[history-call]], [[query-history]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
