---
id: engine-review-items
type: summary
status: unverified
title: Engine::review_items
source: myrn summarize
about: sym:crates/myrn-engine/src/review.rs#review::Engine::review_items
anchors:
- sym:crates/myrn-engine/src/review.rs#review::Engine::review_items@79a3bfa9a1793f1c
map_version: '2'
---
# Engine::review_items

Loads every note, shared notes first, skips duplicate ids and project-map summaries, and attaches the symbol, test or file nodes each note is anchored to or links to via 'about' or 'located_in'.

## Behaviour

- two rows with the same note id -> only the first is kept
- a row whose JSON does not parse -> silently skipped
- a linked node already anchored is not added twice; a linked-only node has no hash, so it is never marked changed

## Terms

export learned notes, anchors, out-links, dedupe, project map, summaries skipped, evidence, linked code

## Connections

- **Location:** `crates/myrn-engine/src/review.rs:49-145`
- **Signature:** `pub fn review_items(&self) -> Result<Vec<ReviewItem>>`
- **In:** [[file-crates-myrn-engine-src-review-rs]]
- **Calls:** [[engine-excerpt]], [[node-location]], [[graph-node-graph]], [[graph-neighbors]], [[note-display-title]]
- **Called by:** [[debug-review]]
