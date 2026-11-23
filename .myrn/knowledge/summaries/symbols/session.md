---
id: session
type: summary
status: unverified
title: session
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#session
anchors:
- sym:crates/myrn-engine/tests/records.rs#session@88a42a0034715153
map_version: '2'
---
# session

Appends four episode entries for the given session id (start, failing pytest run, file_edit of pkg/recipes.py on unique_everseen, passing pytest run) and then reindexes episodes.

## Behaviour

- id 's1' -> four entries appended to the s1 episode log, then reindex
- first pytest run has test 'fail', second has test 'pass'
- append error -> panic

## Terms

recorded session, episode log, test run, failing then passing, replay events, fixture episode

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:75-117`
- **Signature:** `fn session(e: &Engine, id: &str)`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-reindex-episodes]], [[episodes-append]], [[layout-episodes-dir]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`, `session_end_asks_once_per_batch_of_edits`, `one_call_records_many_items_and_trims_test_runs`
