---
id: indexer-gc-modules
type: summary
status: unverified
title: indexer::gc_modules
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::gc_modules
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::gc_modules@6023b68db294603f
---
# indexer::gc_modules

Repeatedly deletes module nodes that have no incoming part_of edge, along with dangling part_of edges, until nothing more is removed.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:348-363`
- **Signature:** `fn gc_modules(conn: &Connection) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Called by:** [[indexer-refresh]]
