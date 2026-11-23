---
id: sync-resolver
type: summary
status: unverified
title: sync::Resolver
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver@623842ab6d03e5ad
map_version: '2'
---
# sync::Resolver

Maps link targets (note ids, node ids, path#qname, commit shas, paths, qualified names) to node ids, using the graph and a lowercase index of note ids.

## Behaviour

- notes maps lowercase note id to the note id as written
- conn is the transaction used for graph lookups
- no state is changed by resolving

## Terms

link resolver, target lookup, id resolution, reference lookup

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:222-226`
- **Signature:** `struct Resolver<'a>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
