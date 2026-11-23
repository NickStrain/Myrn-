---
id: session-history
type: summary
status: unverified
title: session
source: myrn summarize
about: sym:crates/myrn-engine/tests/history.rs#session
anchors:
- sym:crates/myrn-engine/tests/history.rs#session@3f583decc36d4215
map_version: '2'
---
# session

Appends three episode entries (session_start, a prompt with text 'chunked drops items', and a file_edit tool event on pkg/more.py) to episode session 'sess-1', then reindexes episodes on the engine. Panics on errors.

## Behaviour

- writes to the episodes dir under session id sess-1 at timestamps 100, 101, 102
- calling it again appends the same three entries again
- the reindex call makes the entries visible to records, trace, and log queries

## Terms

episode log, session transcript, agent events, prompt event, edit event, reindex episodes, fixture session

## Connections

- **Location:** `crates/myrn-engine/tests/history.rs:56-84`
- **Signature:** `fn session(e: &mut Engine)`
- **In:** [[file-crates-myrn-engine-tests-history-rs]]
- **Calls:** [[engine-reindex-episodes]], [[episodes-append]], [[layout-episodes-dir]]
- **Tests:** `log_lists_records_and_trace_shows_where_they_came_from`
