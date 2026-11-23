---
id: seed-drop-shadowed-stubs
type: summary
status: unverified
title: seed::drop_shadowed_stubs
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::drop_shadowed_stubs
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::drop_shadowed_stubs@a79a6a11af274bff
map_version: '2'
---
# seed::drop_shadowed_stubs

Removes in place each stub node whose qualified name also appears on a non-stub node in the same list, so a function is not listed twice.

## Behaviour

- stub and real node both with qualified name m.f -> stub removed, real node kept
- stub with no real counterpart -> kept
- nodes without a qualified name are never removed

## Terms

duplicate, shadowed, stub duplicate, dedupe, declaration duplicate

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:83-93`
- **Signature:** `pub fn drop_shadowed_stubs(nodes: &mut Vec<Node>)`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[engine-context-excluding]], [[seed-find]]
