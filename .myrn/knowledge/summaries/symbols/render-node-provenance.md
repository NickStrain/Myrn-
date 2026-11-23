---
id: render-node-provenance
type: summary
status: unverified
title: render::node_provenance
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::node_provenance
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::node_provenance@4802dde5d9114504
map_version: '2'
---
# render::node_provenance

Parses a node's extra JSON, using an empty value if it is absent or invalid, then returns the provenance phrase for that node.

## Behaviour

- extra None -> 'private'
- extra '{"written_by":"codex"}' -> 'private, written by codex'
- extra not valid JSON -> 'private', the same as an empty object

## Terms

node origin, record source, who wrote node, attribution lookup, metadata parse

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:87-94`
- **Signature:** `pub fn node_provenance(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[render-provenance]]
- **Called by:** [[engine-known-about]]
