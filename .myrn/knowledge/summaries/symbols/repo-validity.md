---
id: repo-validity
type: summary
status: unverified
title: Repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#Repo
anchors:
- sym:crates/myrn-engine/tests/validity.rs#Repo@c3a7025dc3d2a554
map_version: '2'
---
# Repo

Holds one test's fixture: the temp dir (kept alive), the repo root path, the Engine handle and the id of the first commit.

## Behaviour

- tmp -> TempDir; dropping Repo deletes the temp directory
- root -> path to tmp/repo, the git working tree
- head -> commit id returned by the 'first' commit made in repo()

## Terms

test fixture, temp repository, engine handle, base commit, fixture struct, test context

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:35-40`
- **Signature:** `struct Repo`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
