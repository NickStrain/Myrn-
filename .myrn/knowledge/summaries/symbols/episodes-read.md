---
id: episodes-read
type: summary
status: unverified
title: episodes::read
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::read
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::read@ac1785512e470a7d
---
# episodes::read

Returns every line that parses as an Entry, in file order. Lines that fail to read or parse, such as a truncated tail after a crash, are silently skipped. Errors only if the file cannot be opened.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:95-102`
- **Signature:** `pub fn read(path: &Path) -> Result<Vec<Entry>>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-capture]], [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-records-request]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`, `append_and_read_round_trip`
