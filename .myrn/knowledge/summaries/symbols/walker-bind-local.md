---
id: walker-bind-local
type: summary
status: unverified
title: Walker::bind_local
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::bind_local
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::bind_local@e0bea5f790b988da
---
# Walker::bind_local

Stores a variable-to-type mapping after stripping `mut` patterns; ignores non-identifier patterns and types that type_name rejects.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:402-416`
- **Signature:** `fn bind_local(&mut self, name: Node<'_>, ty: Node<'_>)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[parse-type-name]]
- **Called by:** [[walker-note-params]], [[walker-note-local]]
