---
id: summarize-title-of
type: summary
status: unverified
title: summarize::title_of
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::title_of
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::title_of@38298fdabf75f9cb
map_version: '2'
---
# summarize::title_of

Gives a node's note heading: 'Project' for the root, 'dir/' for a directory, the path for a file, and the last two qname segments joined by '::' for a symbol.

## Behaviour

- module path '' or missing -> 'Project'; module 'crates/x' -> 'crates/x/'
- symbol qname 'a::b::c' -> 'b::c'; a one-segment name is returned as is
- file path is returned unchanged

## Terms

note heading, display name, node title, heading text, symbol name

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:905-917`
- **Signature:** `fn title_of(n: &Node) -> String`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[seed-segments]]
- **Called by:** [[engine-summarize]], [[engine-map-body]]
