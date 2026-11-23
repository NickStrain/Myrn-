---
id: render-one-line
type: summary
status: unverified
title: render::one_line
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::one_line
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::one_line@30aedeae328da7ca
map_version: '2'
---
# render::one_line

Renders one node as a bullet entry with a pointer and, when a summary or doc exists, its first sentence after an em dash. Commit and file nodes use their own shapes, and the result always ends with a newline.

## Behaviour

- commit node, qname abc1, title Fix -> '- commit abc1 Fix' plus newline; summary and doc are ignored
- doc 'Parses input. Then more.' -> ' — Parses input'; split at '. ' so the period is dropped; clipped to 200
- missing qname or title on a commit, or path on a file -> empty string in its place

## Terms

summary text, index entry, pointer, bullet, first sentence, commit entry, file pointer

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:15-39`
- **Signature:** `pub fn one_line(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[episode-clip]], [[node-location]], [[node-display-name]]
- **Called by:** [[render-pack]]
