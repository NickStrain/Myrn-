---
id: engine-excerpt
type: summary
status: unverified
title: Engine::excerpt
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Engine::excerpt
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Engine::excerpt@b8529ff9acd9e76c
---
# Engine::excerpt

Returns the node's source text read from disk under the engine root, or an empty string if the node has no path or the file cannot be read. Non-file nodes use their start and end lines, file nodes use the first 40 lines, and the result is capped at 40 lines with a trailing ellipsis only for non-file nodes that run past the cap.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:143-164`
- **Signature:** `fn excerpt(&self, n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Calls:** [[engine-root]]
- **Called by:** [[engine-review-items]]
