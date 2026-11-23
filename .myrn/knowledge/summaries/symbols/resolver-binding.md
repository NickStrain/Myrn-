---
id: resolver-binding
type: summary
status: unverified
title: Resolver::binding
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::binding
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::binding@dc8851a1d68ce84c
map_version: '2'
---
# Resolver::binding

Returns the import binding for a local name in the given file, if one was recorded.

## Behaviour

- file with no bindings -> None
- name not imported in that file -> None
- imported name -> the stored Binding

## Terms

import binding lookup, alias lookup, local name, imported name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:491-493`
- **Signature:** `fn binding(&self, file: &str, name: &str) -> Option<&Binding>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-follow-rust]], [[resolver-reexported]], [[resolver-exported-from]], [[resolver-resolve-method]]
