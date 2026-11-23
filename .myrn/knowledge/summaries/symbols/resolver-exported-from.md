---
id: resolver-exported-from
type: summary
status: unverified
title: Resolver::exported_from
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::exported_from
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::exported_from@3d0ea24c2a7e5310
map_version: '2'
---
# Resolver::exported_from

Finds a name as seen from outside a file: a top-level symbol, then a named re-export, then a wildcard re-export, recursing up to depth.

## Behaviour

- top-level symbol with the name in the file -> that symbol
- name only available through a wildcard re-export -> followed into that target
- depth 0 and no own symbol -> empty

## Terms

re-export, wildcard re-export, star export, top-level symbol, module export, public name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:687-713`
- **Signature:** `fn exported_from(&self, file: &str, name: &str, depth: usize) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-lookup]], [[resolver-binding]], [[resolver-exported-from]], [[resolver-is-top-level]], [[resolver-in-file]]
- **Called by:** [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
