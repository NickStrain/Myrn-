---
id: resolver-exported-from
type: summary
status: unverified
title: Resolver::exported_from
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::exported_from
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::exported_from@3d0ea24c2a7e5310
---
# Resolver::exported_from

Finds a name as seen from outside a file: a top-level symbol, a named re-export or a wildcard re-export, recursing through glob targets up to the given depth.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:687-713`
- **Signature:** `fn exported_from(&self, file: &str, name: &str, depth: usize) -> Vec<usize>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[resolver-lookup]], [[resolver-binding]], [[resolver-exported-from]], [[resolver-is-top-level]], [[resolver-in-file]]
- **Called by:** [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
