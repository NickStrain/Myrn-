---
id: engine-excerpt
type: summary
status: unverified
title: Engine::excerpt
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Engine::excerpt
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Engine::excerpt@b8529ff9acd9e76c
map_version: '2'
---
# Engine::excerpt

Reads the node's file and returns the lines of its span, capped at 40, with an ellipsis line appended when a symbol's span is cut short.

## Behaviour

- node with no path, or unreadable file -> empty string
- symbol or test with start and end lines -> that line range, with start clamped to at least line 1
- span longer than 40 lines -> first 40 lines plus an ellipsis line; file nodes get the first 40 lines and never an ellipsis

## Terms

code snippet, source excerpt, line range, truncation, span, code preview, line cap

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:147-168`
- **Signature:** `fn excerpt(&self, n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Calls:** [[engine-root]]
- **Called by:** [[engine-review-items]]
