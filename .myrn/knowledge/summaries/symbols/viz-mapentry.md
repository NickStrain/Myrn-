---
id: viz-mapentry
type: summary
status: unverified
title: viz::MapEntry
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::MapEntry
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::MapEntry@54a658f698574247
map_version: '2'
---
# viz::MapEntry

One project map tree entry (directory, file or symbol) with its summary, line, freshness state and nested children.

## Behaviour

- state is one of fresh, stale or missing
- name is the display name for symbols and the title or id otherwise
- children start empty and are filled by the parent pass in viz_map

## Terms

tree node, map item, freshness, nested children, summary state

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:54-67`
- **Signature:** `pub struct MapEntry`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
