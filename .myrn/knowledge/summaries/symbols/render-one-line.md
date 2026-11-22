---
id: render-one-line
type: summary
status: unverified
title: render::one_line
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::one_line
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::one_line@30aedeae328da7ca
---
# render::one_line

Renders one node as a single '- ' line: commits show qname and title, files show path, and other nodes show kind, display_name, and location, followed by the first sentence of summary (falling back to doc) clipped to 200 chars. An empty Some summary blocks the doc fallback because only None falls through, and commits never get a doc suffix.

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:14-38`
- **Signature:** `pub fn one_line(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[episode-clip]], [[node-location]], [[node-display-name]]
- **Called by:** [[render-pack]]
