---
id: walker-push-def
type: summary
status: unverified
title: Walker::push_def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_def@dee20674c668d4d0
---
# Walker::push_def

Creates a Def with a unique qname (appends '#N' for duplicates), computes signature, doc, and body hash, and returns its index in out.defs.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:470-504`
- **Signature:** `fn push_def( &mut self, node: Node<'_>, name: String, kind: &'static str, is_test: bool, body: Option<Node<'_>>, doc_anchor: Option<Node<'_>>, ) -> usize`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[body-hash-body-hash]], [[lang-sep]], [[walker-line]], [[walker-rust-mod-path]], [[walker-signature]], [[walker-doc]]
- **Called by:** [[walker-visit]]
