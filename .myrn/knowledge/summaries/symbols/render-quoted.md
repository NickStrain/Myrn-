---
id: render-quoted
type: summary
status: unverified
title: render::quoted
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::quoted
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::quoted@af83fad848747ccd
---
# render::quoted

Renders a learned record as a `> id · type · status[ · from source]: title` header line, followed by one quoted body line made of the doc's non-empty lines (trimmed, with lines equal to the title removed) joined by spaces and clipped to 280 chars. Status defaults to `unverified`; source is read by parsing the node's `extra` JSON and is omitted if it is absent or unparseable.

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:44-75`
- **Signature:** `pub fn quoted(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[render-pack]]
