---
id: summarize-filejob
type: summary
status: unverified
title: summarize::FileJob
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::FileJob
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::FileJob@5c26259e2e332353
map_version: '2'
---
# summarize::FileJob

One file to send to the model: its node id, path, language and the stale symbol nodes to describe, built only when the file or one of its symbols is stale.

## Behaviour

- symbols lists only the stale symbols of the file, not every symbol
- file stale but no symbol stale -> symbols is empty and the prompt asks for an empty symbols object
- built from the file nodes in graph order

## Terms

work item, file request, batch unit, stale file job, model job

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:625-630`
- **Signature:** `struct FileJob`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
