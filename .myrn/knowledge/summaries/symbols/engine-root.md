---
id: engine-root
type: summary
status: unverified
title: Engine::root
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::root
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::root@d563d4930e1c20fd
map_version: '2'
---
# Engine::root

Returns the repository root path.

## Behaviour

- Returns the layout repo root

## Terms

repository path, working tree, project root

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:247-249`
- **Signature:** `pub fn root(&self) -> &Path`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Called by:** [[engine-source-if-short]], [[engine-doctor]], [[engine-trace]], [[engine-diff-knowledge]], [[engine-excerpt]], [[engine-history-messages]], [[connect-run]], [[disconnect-run]], [[doctor-agent-checks]], [[hook-handle]], [[init-run]], [[publish-run]], [[ui-run]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`, `diff_compares_shared_knowledge_between_points`, `published_record_reaches_a_teammate_through_git`, `map_is_written_linked_incremental_and_searchable`
