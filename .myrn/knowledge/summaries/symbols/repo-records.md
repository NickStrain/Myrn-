---
id: repo-records
type: summary
status: unverified
title: Repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#Repo
anchors:
- sym:crates/myrn-engine/tests/records.rs#Repo@5e0b359e24a25cf8
map_version: '2'
---
# Repo

Holds a test fixture: the TempDir that owns the files, the repo root path, the Engine, and the head commit id. _tmp is kept only so the directory is not deleted early.

## Behaviour

- _tmp -> dropped only when the Repo is dropped, keeping root valid
- head -> full hex id of the single commit made by repo()

## Terms

temp directory owner, test fixture, commit id, engine handle, repo root, fixture bundle

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:43-48`
- **Signature:** `struct Repo`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
