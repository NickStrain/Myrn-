---
id: pull-keeping-local
type: summary
status: unverified
title: pull_keeping_local
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#pull_keeping_local
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#pull_keeping_local@ef7e8122a3ad59a9
map_version: '2'
---
# pull_keeping_local

Fetches the current branch from origin, safely checks out origin/<branch>'s tree without overwriting modified tracked files, and detaches HEAD at that commit. Unlike pull, it keeps untracked local files and does not move the branch ref.

## Behaviour

- untracked local files that do not conflict are kept
- local edits to a file the incoming tree changes -> checkout error and panic
- HEAD ends detached, so later commits do not advance the branch

## Terms

keep untracked, detached head, safe checkout, merge-like pull, local changes, pull without reset, preserve local files

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:306-322`
- **Signature:** `fn pull_keeping_local(root: &Path)`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Tests:** `shared_records_that_disagree_are_flagged_for_a_person`
