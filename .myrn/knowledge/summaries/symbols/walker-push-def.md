---
id: walker-push-def
type: summary
status: unverified
title: Walker::push_def
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_def
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_def@dee20674c668d4d0
map_version: '2'
---
# Walker::push_def

Appends a Def for a node, building its qualified name, signature, doc, body hash and line span, and returns its index.

## Behaviour

- Rust impl Wrapper { fn fmt } gives qname Wrapper::fmt, using the language separator
- A second Wrapper::fmt in the same file gets Wrapper::fmt#2
- The qname is the file module prefix, then enclosing scope names, then the def name

## Terms

add definition, qualified name, dedupe, def record, line span

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:470-504`
- **Signature:** `fn push_def( &mut self, node: Node<'_>, name: String, kind: &'static str, is_test: bool, body: Option<Node<'_>>, doc_anchor: Option<Node<'_>>, ) -> usize`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[body-hash-body-hash]], [[lang-sep]], [[walker-line]], [[walker-rust-mod-path]], [[walker-signature]], [[walker-doc]]
- **Called by:** [[walker-visit]]
