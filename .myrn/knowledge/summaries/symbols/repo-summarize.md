---
id: repo-summarize
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#repo
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#repo@ca818524d3ba37df
map_version: '2'
---
# repo

Builds a temp git repo at tmp/repo with the given files committed as HEAD "first", then initialises an Engine with home at tmp/home; returns the TempDir (which owns and deletes the directory) and the Engine.

## Behaviour

- [("src/lib.rs", "x")] -> repo with one root commit containing src/lib.rs
- commit author is fixed as "t" <t@example.invalid>
- TempDir dropped -> the repo directory is deleted, so keep it alive

## Terms

temp repository, git fixture, initial commit, test repo, engine setup, test workspace

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:19-34`
- **Signature:** `fn repo(files: &[(&str, &str)]) -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Calls:** [[engine-init]], [[write-summarize]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
