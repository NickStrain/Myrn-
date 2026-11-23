---
id: node-origin
type: summary
status: unverified
title: node::Origin
source: myrn summarize
about: sym:crates/myrn-model/src/node.rs#node::Origin
anchors:
- sym:crates/myrn-model/src/node.rs#node::Origin@6a404c2865f185e7
map_version: '2'
---
# node::Origin

Two-valued marker for whether a node is computed from code (Derived) or comes from agent sessions and is stored durably (Earned).

## Behaviour

- Only Derived and Earned variants exist
- Serializes as "derived" or "earned" via serde snake_case
- Derives Copy, Eq and Hash so it can be used as a map key

## Terms

provenance, derived vs earned, computed, rebuildable, agent session, durable, origin kind

## Connections

- **Location:** `crates/myrn-model/src/node.rs:8-11`
- **Signature:** `pub enum Origin`
- **In:** [[file-crates-myrn-model-src-node-rs]]
