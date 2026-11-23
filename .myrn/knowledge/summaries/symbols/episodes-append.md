---
id: episodes-append
type: summary
status: unverified
title: episodes::append
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::append
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::append@7f7971a1af96cfc0
map_version: '2'
---
# episodes::append

Creates the directory if needed, appends one JSON line for the entry to the session's log, and returns the file path.

## Behaviour

- new session creates the file; existing session adds a line at the end
- file is opened create+append, never truncated; each line is written in a single write_all call
- directory creation, serialization, or open failure returns Err with 'opening <path>' context

## Terms

write, log entry, persist, record event, concurrent writes, add line, save

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:80-92`
- **Signature:** `pub fn append(dir: &Path, session: &str, entry: &Entry) -> Result<PathBuf>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Calls:** [[episodes-path-for]]
- **Called by:** [[engine-capture]], [[engine-records-request]], [[session-history]], [[session]]
- **Tests:** `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `sessions_attach_their_outcome`, `session_end_asks_once_per_batch_of_edits`, `records_become_verified_when_a_cited_test_passes_or_the_fix_lands`, `append_and_read_round_trip`
