---
id: episodes-file-name
type: summary
status: unverified
title: episodes::file_name
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::file_name
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::file_name@eab94b1f74a99654
map_version: '2'
---
# episodes::file_name

Turns a session id into a safe '<id>.jsonl' filename by replacing characters outside ASCII letters, digits, '-' and '_' with '_', and truncating to 128 characters.

## Behaviour

- '../../etc/passwd' -> '______etc_passwd.jsonl' (path traversal blocked)
- '' -> 'unknown.jsonl'
- 'abc-123_x' -> 'abc-123_x.jsonl' (unchanged)

## Terms

sanitize, session id, path traversal, filename, slug, escape, truncate

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:57-73`
- **Signature:** `pub fn file_name(session: &str) -> String`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-episode-log]], [[engine-current-session]], [[episodes-path-for]]
- **Tests:** `session_ids_cannot_escape_the_directory`
