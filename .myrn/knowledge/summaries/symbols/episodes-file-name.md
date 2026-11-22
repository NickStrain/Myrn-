---
id: episodes-file-name
type: summary
status: unverified
title: episodes::file_name
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::file_name
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::file_name@eab94b1f74a99654
---
# episodes::file_name

Maps a session id to a safe filename by replacing any character outside ASCII alphanumerics, '-' and '_' with '_', truncating to 128 chars, and returning 'unknown.jsonl' for empty input. Pure function with no I/O; this is what stops agent-supplied ids from escaping the directory.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:57-73`
- **Signature:** `pub fn file_name(session: &str) -> String`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-episode-log]], [[engine-current-session]], [[episodes-path-for]]
- **Tests:** `session_ids_cannot_escape_the_directory`
