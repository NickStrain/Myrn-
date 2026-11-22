---
id: episodes-list
type: summary
status: unverified
title: episodes::list
source: myrn summarize
about: sym:crates/myrn-store/src/episodes.rs#episodes::list
anchors:
- sym:crates/myrn-store/src/episodes.rs#episodes::list@fa24eacfa64a9748
---
# episodes::list

Returns (id, path) pairs for every *.jsonl file in dir, sorted by modification time descending with ties broken by id ascending. The id is the file stem, so it is the sanitized form and not the original session id. Returns an empty Vec if dir is unreadable (including when it does not exist), and silently skips files whose metadata or mtime can't be read.

## Connections

- **Location:** `crates/myrn-store/src/episodes.rs:105-121`
- **Signature:** `pub fn list(dir: &Path) -> Result<Vec<(String, PathBuf)>>`
- **In:** [[file-crates-myrn-store-src-episodes-rs]]
- **Called by:** [[engine-reindex-episodes]], [[engine-episodes]], [[engine-episode-log]], [[engine-recorded-test-run]], [[engine-current-session]], [[log-render-one]]
- **Tests:** `append_and_read_round_trip`
