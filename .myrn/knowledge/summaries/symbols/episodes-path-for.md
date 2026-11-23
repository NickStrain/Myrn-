---
id: episodes-path-for
type: summary
status: unverified
title: episodes::path_for
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::path_for
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::path_for@e525e209f9043165
map_version: '2'
---
# episodes::path_for

Joins the episodes directory with the sanitized filename for a session id, without touching the filesystem.

## Behaviour

- dir '/x' and session 's1' -> '/x/s1.jsonl'
- unsafe characters are replaced first, so the result stays inside dir
- does not create the directory or file

## Terms

log path, session file, jsonl path, location, episode path

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:75-77`
- **Signature:** `pub fn path_for(dir: &Path, session: &str) -> PathBuf`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Calls:** [[episodes-file-name]]
- **Called by:** [[engine-capture]], [[engine-records-request]], [[engine-session-agent]], [[episodes-append]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`, `append_and_read_round_trip`
