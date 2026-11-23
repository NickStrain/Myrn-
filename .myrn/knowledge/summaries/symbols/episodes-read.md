---
id: episodes-read
type: summary
status: unverified
title: episodes::read
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::read
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::read@ac1785512e470a7d
map_version: '2'
---
# episodes::read

Parses every line of a log file into Entry values, skipping lines that fail to parse.

## Behaviour

- a truncated or malformed line (e.g. from a crash) is skipped and the rest are returned
- missing file -> Err with 'opening <path>' context; empty file -> empty Vec
- map_while stops at the first I/O read error, so lines after it are dropped, not just skipped

## Terms

load, parse, deserialize, replay, crash recovery, corrupt line, history

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:95-102`
- **Signature:** `pub fn read(path: &Path) -> Result<Vec<Entry>>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-records-request]], [[engine-session-agent]], [[engine-gc]], [[engine-passing-test-runs]]
- **Tests:** `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `shell_edits_are_captured_and_lead_to_a_record_request`, `append_and_read_round_trip`
