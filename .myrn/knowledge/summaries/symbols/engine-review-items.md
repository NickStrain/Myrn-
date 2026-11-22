---
id: engine-review-items
type: summary
status: unverified
title: Engine::review_items
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Engine::review_items
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Engine::review_items@0cd6a3b33b9eaddd
---
# Engine::review_items

Reads every row of the notes table (shared first, then by path), skipping rows that fail to parse and repeated ids (the first one wins). For each note it gathers code targets from anchors plus outgoing 'about' and 'located_in' edges, keeps only symbol, test, and file nodes, and renders links as '<target> fixes this' for inverse links or '<rel> <target>' otherwise. Returns records even when they have no code targets; origin is left empty.

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:49-141`
- **Signature:** `pub fn review_items(&self) -> Result<Vec<ReviewItem>>`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Calls:** [[engine-excerpt]], [[node-location]], [[graph-node-graph]], [[graph-neighbors]], [[note-display-title]]
- **Called by:** [[debug-review]]
