---
id: repo
type: summary
status: unverified
title: Repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/notes.rs#Repo
anchors:
- sym:crates/myrn-engine/tests/notes.rs#Repo@13a2f6b176d43e9a
map_version: '2'
---
# Repo

Test context holding the temp directory, the repo root path, the Engine opened on it, and the initial commit id.

## Behaviour

- _tmp field -> the TempDir is deleted when the Repo is dropped
- head -> full commit id of the single 'first' commit with no parent
- root -> the repo directory inside the temp dir

## Terms

test context, fixture handle, temp repo, engine handle

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs:38-43`
- **Signature:** `struct Repo`
- **In:** [[file-crates-myrn-engine-tests-notes-rs]]
