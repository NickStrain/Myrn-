---
id: sync-segments
type: summary
status: unverified
title: sync::segments
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::segments
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::segments@3ddf7aea8186c394
map_version: '2'
---
# sync::segments

Splits a qualified name into its parts, treating '::' and '.' as separators and dropping empty parts.

## Behaviour

- 'a::b.c' gives ['a', 'b', 'c']
- 'a..b' gives ['a', 'b']
- '::x' gives ['x']

## Terms

split qualified name, name parts, tokenize, qname segments

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:228-234`
- **Signature:** `fn segments(q: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[resolver-resolve-sync]]
