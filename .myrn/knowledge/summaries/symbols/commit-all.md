---
id: commit-all
type: summary
status: unverified
title: commit_all
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#commit_all
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#commit_all@cb3d1f6b31a9bcbe
map_version: '2'
---
# commit_all

Opens the git repo at dir, or initialises one if none exists, then stages all files and commits them to HEAD with the Gate Zero signature.

## Behaviour

- dir not a repo -> git init, then a root commit with no parent
- second call -> commit whose parent is the current HEAD
- files deleted from disk are removed from the index via update_all before committing

## Terms

git commit, stage all, init repo, snapshot, fixture history

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:30-44`
- **Signature:** `fn commit_all(dir: &Path, message: &str)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[setup]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
