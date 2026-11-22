---
id: commit-all
type: summary
status: unverified
title: commit_all
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#commit_all
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#commit_all@cb3d1f6b31a9bcbe
---
# commit_all

Stages all files in dir (add_all and update_all with '*') and commits them to HEAD as 'Gate Zero'. Opens the repo or initializes it if absent, and parents the commit on the current HEAD when one exists, so repeated calls build history; the first call creates the root commit.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:30-44`
- **Signature:** `fn commit_all(dir: &Path, message: &str)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[setup]]
- **Tests:** `incremental_refresh_tracks_edits_and_deletes`
