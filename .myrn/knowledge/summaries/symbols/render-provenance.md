---
id: render-provenance
type: summary
status: unverified
title: render::provenance
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::provenance
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::provenance@223524b160b7e055
map_version: '2'
---
# render::provenance

Builds a short origin phrase from a record's extra JSON, saying whether it is shared or private, who published or wrote it, and when. Empty strings count as missing.

## Behaviour

- visibility shared, published_by Ana, published_at 2026-11-22, written_by claude, created 2026-11-20 -> 'shared by Ana on 2026-11-22, written by claude on 2026-11-20'
- empty object -> 'private'
- visibility shared with no publisher -> 'shared by a teammate'; only source set, no writer or date -> 'private, from import'

## Terms

record origin, attribution, author, publisher, who wrote, shared or private, date stamp

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:46-68`
- **Signature:** `pub fn provenance(extra: &serde_json::Value) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[visibility-as-str]]
- **Called by:** [[render-node-provenance]], [[render-quoted]], [[engine-recall]]
