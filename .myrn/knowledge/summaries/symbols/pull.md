---
id: pull
type: summary
status: unverified
title: pull
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#pull
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#pull@4a3873baa0e40c04
map_version: '2'
---
# pull

Fetches the current branch from origin, then hard-resets HEAD and the working tree to origin/<branch>. Despite the doc comment saying fast-forward, it overwrites rather than checking for fast-forward.

## Behaviour

- local uncommitted edits to tracked files are discarded
- local commits on the branch not on origin are dropped from the branch
- no origin/<branch> ref after fetch -> panic via unwrap

## Terms

fetch and reset, sync, update from remote, hard reset, fast-forward, git pull, discard local changes

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:51-65`
- **Signature:** `fn pull(root: &Path)`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`
