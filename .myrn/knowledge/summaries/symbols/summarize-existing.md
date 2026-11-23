---
id: summarize-existing
type: summary
status: unverified
title: summarize::Existing
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Existing
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Existing@7ba51e2445c0f94f
map_version: '2'
---
# summarize::Existing

A summary note loaded from an earlier run, keyed by the node it is about, with its id, text, behaviour, terms, map version, anchor hash and file path.

## Behaviour

- only notes with type summary, source 'myrn summarize' and an About link are loaded; others are ignored
- version is the map_version extra field; absent -> None, which makes a symbol stale
- hash is the first anchor's hash; None when that anchor has no hash

## Terms

previous note, saved summary, prior run, note index, map version, anchor hash

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:95-103`
- **Signature:** `struct Existing`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
