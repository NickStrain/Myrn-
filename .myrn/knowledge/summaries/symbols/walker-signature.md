---
id: walker-signature
type: summary
status: unverified
title: Walker::signature
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::signature
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::signature@6301e1cf2d8121c2
map_version: '2'
---
# Walker::signature

Builds a single-line header from the node start up to its body, collapsing whitespace and removing trailing '{', ':', '=' or '=>'.

## Behaviour

- Rust 'pub fn f(x: u8) -> u8 {' gives 'pub fn f(x: u8) -> u8'
- Python 'def f(x):' gives 'def f(x)'
- With no body, or a body starting before the node, it uses the first line; output is capped at 200 characters

## Terms

function header, declaration line, signature text, header, prototype

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:506-527`
- **Signature:** `fn signature(&self, node: Node<'_>, body: Option<Node<'_>>) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[parse-truncate]]
- **Called by:** [[walker-push-def]]
