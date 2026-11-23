---
id: knowledge-tracedanchor
type: summary
status: unverified
title: knowledge::TracedAnchor
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::TracedAnchor
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::TracedAnchor@3a1544869259c5d4
map_version: '2'
---
# knowledge::TracedAnchor

One code anchor of a record, with the symbol location it resolves to and a state of `current`, `changed` or `gone`.

## Behaviour

- no resolved symbol, or symbol missing from the graph -> state `gone` and location None
- resolved symbol listed in sync drift for this note -> state `changed`
- otherwise state `current` with the node's location

## Terms

anchor, code link, symbol, drift, stale anchor, moved symbol

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:70-75`
- **Signature:** `pub struct TracedAnchor`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
