---
id: render-quoted
type: summary
status: unverified
title: render::quoted
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/render.rs#pack::render::quoted
anchors:
- sym:crates/myrn-engine/src/pack/render.rs#pack::render::quoted@32b09e7c927368df
map_version: '2'
---
# render::quoted

Formats a learned record as a quoted block: a header with id, type, status and provenance, then the title, then the body with repeated heading lines removed, clipped to 280 characters. A status of suspect adds a possibly-outdated note to the header.

## Behaviour

- status absent -> header shows 'unverified'; status suspect -> adds '(possibly outdated: the code changed since)'
- doc first line '# Title' equal to the title -> that line dropped; remaining non-empty lines joined with spaces and quoted on a second line
- doc empty or only repeats the title -> output is just the header line with the title

## Terms

learned note, memory record, past session, quote, suspect status, outdated marker, unverified

## Connections

- **Location:** `crates/myrn-engine/src/pack/render.rs:97-130`
- **Signature:** `pub fn quoted(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-render-rs]]
- **Calls:** [[episode-clip]], [[render-provenance]], [[render-safe-text]]
- **Called by:** [[render-pack]], [[engine-suspect-notice]]
