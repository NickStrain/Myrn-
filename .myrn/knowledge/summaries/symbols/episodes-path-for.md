---
id: episodes-path-for
type: summary
status: unverified
title: episodes::path_for
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::path_for
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::path_for@e525e209f9043165
---
# episodes::path_for

Joins dir with file_name(session) to produce the log path. Pure path computation that does not touch the filesystem.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:75-77`
- **Signature:** `pub fn path_for(dir: &Path, session: &str) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Calls:** [[episodes-file-name]]
- **Called by:** [[engine-capture]], [[engine-records-request]], [[episodes-append]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`, `append_and_read_round_trip`
