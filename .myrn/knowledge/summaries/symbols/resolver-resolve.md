---
id: resolver-resolve
type: summary
status: unverified
title: Resolver::resolve
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::resolve@355bc3a023517900
map_version: '2'
---
# Resolver::resolve

Resolves one raw reference from the source symbol into target indexes with edge status, dispatching on reference kind and removing duplicate targets.

## Behaviour

- kind 'call' -> resolve_bare; 'path' -> resolve_path; 'method' -> resolve_method
- unknown kind text -> treated as Call
- same target found twice -> returned once

## Terms

reference resolution, call resolution, dispatch, deduplicate targets, edge status

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:468-485`
- **Signature:** `pub fn resolve(&self, src: usize, r: &RawRef) -> Vec<(usize, EdgeStatus)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[refkind-parse]], [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
- **Called by:** [[symbols-resolve-all]], [[tests-targets]]
