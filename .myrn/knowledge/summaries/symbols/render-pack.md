---
id: render-pack
type: summary
status: unverified
title: render::pack
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::pack
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::pack@6c02642e520303fb
---
# render::pack

Assembles the pack string in order: HEADER, then each card as `\n` + card, then the `Also relevant:` section with one_line for each rest node, then the KNOWN_HEADER section with quoted for each known node; each piece is added only if it fits the Budget. Returns (text, ids of items that fit), and returns an empty string and empty ids if nothing was added or if all inputs are empty or the HEADER itself does not fit.

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:80-114`
- **Signature:** `pub fn pack( cards: &[(String, String)], rest: &[Node], known: &[Node], budget: usize, ) -> (String, Vec<String>)`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[budget-new]], [[budget-push]], [[render-one-line]], [[render-quoted]]
- **Called by:** [[engine-context-excluding]]
