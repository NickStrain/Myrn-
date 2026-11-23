---
id: episodes-list
type: summary
status: unverified
title: episodes::list
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::list
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::list@fa24eacfa64a9748
map_version: '2'
---
# episodes::list

Returns (session id, path) for each .jsonl file in the directory, sorted newest modification time first, with ties broken by id ascending.

## Behaviour

- missing or unreadable directory -> Ok(empty Vec), not an error
- the id is the file stem, i.e. the sanitized filename, which can differ from the original session id
- non-.jsonl files are ignored; files whose metadata or mtime cannot be read are dropped silently

## Terms

sessions, recent, history, ordering, enumerate, sort by time, listing

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:105-121`
- **Signature:** `pub fn list(dir: &Path) -> Result<Vec<(String, PathBuf)>>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-current-session]], [[engine-gc]], [[engine-passing-test-runs]], [[log-render-one]]
- **Tests:** `append_and_read_round_trip`
