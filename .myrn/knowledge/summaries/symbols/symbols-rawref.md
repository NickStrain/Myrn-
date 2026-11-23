---
id: symbols-rawref
type: summary
status: unverified
title: symbols::RawRef
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::RawRef
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::RawRef@b8ee4db98c67f524
map_version: '2'
---
# symbols::RawRef

One row from the refs table: the source symbol id, file path, reference kind, name, optional qualifier and optional scope.

## Behaviour

- kind 'import' rows are handled by add_import, not resolve
- kind values parsed by RefKind; an unknown kind falls back to Call
- qualifier holds text such as 'Self', 'self' or a receiver

## Terms

raw reference, refs row, call site, unresolved reference, reference record

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:80-87`
- **Signature:** `pub struct RawRef`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
