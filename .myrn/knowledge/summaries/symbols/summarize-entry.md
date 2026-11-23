---
id: summarize-entry
type: summary
status: unverified
title: summarize::Entry
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::Entry
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::Entry@bfb078a7fb7949a9
map_version: '2'
---
# summarize::Entry

One map node (directory, file or symbol) with its body hash, note id, current summary, behaviour, terms and a stale flag that says it must be asked again.

## Behaviour

- file or symbol hash is the node body_hash; directory hash is 16 hex chars over its children's ids and summaries
- stale is always false for a directory entry
- summary is None when no earlier note holds a non-empty summary and the model has not answered this run

## Terms

map entry, stale flag, body hash, note id, cached summary, map node

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:81-92`
- **Signature:** `struct Entry`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
