---
id: indexer-gc-modules
type: summary
status: unverified
title: indexer::gc_modules
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::gc_modules
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::gc_modules@6023b68db294603f
map_version: '2'
---
# indexer::gc_modules

Deletes module nodes that have no incoming part_of edge, and part_of edges whose source no longer exists, repeating until a pass deletes nothing so empty parent directories are removed too.

## Behaviour

- directory with no child files or subdirectories -> module node removed
- module with a remaining part_of child -> kept
- called from refresh inside the same transaction, after file changes

## Terms

prune, orphan, empty directory, cascade, dangling edges, garbage collect

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:348-363`
- **Signature:** `fn gc_modules(conn: &Connection) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Called by:** [[indexer-refresh]]
