---
id: walker-signature
type: summary
status: unverified
title: Walker::signature
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::signature
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::signature@6301e1cf2d8121c2
---
# Walker::signature

Builds a single-line signature from the start of the node up to the body (or first newline), collapsing whitespace, trimming trailing '{', ':', '=', or '=>', and truncating to 200 chars.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:506-527`
- **Signature:** `fn signature(&self, node: Node<'_>, body: Option<Node<'_>>) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-text]], [[parse-truncate]]
- **Called by:** [[walker-push-def]]
