---
id: engine-trace
type: summary
status: unverified
title: Engine::trace
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::trace
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Engine::trace@67baa3f141a4d4dd
map_version: '2'
---
# Engine::trace

Builds the Trace for one id by parsing its source session, resolving cited commits and tests, checking each anchor against the graph and drift, listing supersede and contradict links, and reading Git history for shared notes.

## Behaviour

- source `kind:ref` -> session id is the part after the first colon; a bare source is used whole
- evidence kind `commit` -> commit listed as `evidence`; `test` and `test_run` kept as raw strings; verified_by links appended to tests
- contradicts lists links in both directions; supersedes lists targets only, superseded_by lists sources only

## Terms

provenance, origin, where it came from, backing, session, evidence, history

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:199-304`
- **Signature:** `pub fn trace(&self, id: &str) -> Result<Trace>`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
- **Calls:** [[kind-as-str]], [[engine-root]], [[engine-episode-log]], [[episode-summary]], [[engine-record-row]], [[knowledge-file-history]], [[visibility-as-str]], [[node-location]], [[graph-node-graph]], [[sync-drift-sync]]
- **Called by:** [[log-run]], [[trace-run]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`
