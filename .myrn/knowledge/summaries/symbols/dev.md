---
id: dev
type: summary
status: unverified
title: Dev
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#Dev
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#Dev@0a12331bd8c541f7
map_version: '2'
---
# Dev

Pairs a developer's git worktree path (root) with the Engine bound to it (e). Tests use it as a and b.

## Behaviour

- root is the repo directory on disk
- e is the engine for that repo with its own private home store
- returned by two_clones as the two developers

## Terms

developer, teammate, repo handle, engine pair, clone wrapper, worktree

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:67-70`
- **Signature:** `struct Dev`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
