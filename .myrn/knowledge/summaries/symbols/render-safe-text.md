---
id: render-safe-text
type: summary
status: unverified
title: render::safe_text
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::safe_text
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::safe_text@d1e896f1ebbb8805
map_version: '2'
---
# render::safe_text

Returns the text unchanged for private records, and passes it through directives::strip for shared records, since another person wrote those. It re-applies stripping because a shared note can be committed without publish.

## Behaviour

- extra visibility shared -> the .text field returned by directives::strip on the input
- extra visibility private or absent -> input returned unchanged
- extra missing or not valid JSON -> treated as not shared, input returned unchanged

## Terms

directive stripping, sanitising, untrusted text, instruction filter, shared records, remove directives

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:73-84`
- **Signature:** `pub fn safe_text(n: &Node, text: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[visibility-as-str]], [[directives-strip]]
- **Called by:** [[engine-known-about]], [[render-quoted]], [[engine-recall]], [[engine-viz-details]]
