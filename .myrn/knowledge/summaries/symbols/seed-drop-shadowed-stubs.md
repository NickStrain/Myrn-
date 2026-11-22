---
id: seed-drop-shadowed-stubs
type: summary
status: unverified
title: seed::drop_shadowed_stubs
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::drop_shadowed_stubs
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::drop_shadowed_stubs@a79a6a11af274bff
---
# seed::drop_shadowed_stubs

Removes stub-file nodes from the vector when a non-stub node with the same qualname is also present, so a function appears once. Mutates in place.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:83-93`
- **Signature:** `pub fn drop_shadowed_stubs(nodes: &mut Vec<Node>)`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[engine-context-excluding]], [[seed-find]]
