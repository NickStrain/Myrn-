---
id: api-located
type: summary
status: unverified
title: api::Located
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Located
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Located@134f97d3f4c2227c
map_version: '2'
---
# api::Located

Repository root, project id, whether the id is provisional, and an optional worktree tag. The tag is the first 12 hex characters of a hash of the root path, and is set only for linked worktrees.

## Behaviour

- worktree is None unless the repository is a linked worktree
- provisional is true only when no id came from config or a root commit

## Terms

repository location, worktree tag, project id, root path

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:140-145`
- **Signature:** `struct Located`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
