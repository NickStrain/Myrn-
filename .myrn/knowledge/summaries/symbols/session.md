---
id: session
type: summary
status: unverified
title: session
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#session
anchors:
- sym:crates/myrn-engine/tests/records.rs#session@88a42a0034715153
---
# session

Appends a four-entry episode under the given id (session_start, failing pytest run, file_edit of pkg/recipes.py with unique_everseen as its symbol, passing pytest run) to the episodes dir, then reindexes episodes.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:75-117`
- **Signature:** `fn session(e: &Engine, id: &str)`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Calls:** [[engine-reindex-episodes]], [[episodes-append]], [[layout-episodes-dir]]
- **Tests:** `bugs_are_stored_deduplicated_and_recalled`, `session_end_asks_once_per_batch_of_edits`, `one_call_records_many_items_and_trims_test_runs`
