---
id: setup-history
type: summary
status: unverified
title: setup
source: myrn summarize
about: sym:crates/myrn-engine/tests/history.rs#setup
anchors:
- sym:crates/myrn-engine/tests/history.rs#setup@6856c453a20d18d9
map_version: '2'
---
# setup

Creates a temp dir with repo/pkg/__init__.py and repo/pkg/more.py (defining chunked and first), inits git and commits it as 'fix chunked', then builds an Engine on that repo with a home dir in the temp dir. Returns the TempDir guard, the Engine, and the commit sha.

## Behaviour

- returns (tmp, engine, head_sha) -> the first element must stay alive, since dropping TempDir deletes the files
- the second value from Engine::init is discarded
- the returned sha is the full 40-char commit id from commit_all

## Terms

fixture repo, test engine, temp repo, init engine, head sha, test scaffold, temp directory

## Connections

- **Location:** `crates/myrn-engine/tests/history.rs:40-54`
- **Signature:** `fn setup() -> (tempfile::TempDir, Engine, String)`
- **In:** [[file-crates-myrn-engine-tests-history-rs]]
- **Calls:** [[engine-init]], [[write-history]], [[commit-all-history]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`, `diff_compares_shared_knowledge_between_points`, `forget_removes_the_record_and_what_only_it_used`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`
