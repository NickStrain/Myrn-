---
id: viz-details
type: summary
status: unverified
title: viz::Details
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::Details
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::Details@dc2f2d23e69de355
map_version: '2'
---
# viz::Details

Side-panel payload for one id: its node, note body and note path, and up to 400 edges split into outgoing and incoming.

## Behaviour

- node is None when the id is not in the store
- body is set only when the note data parses, and is passed through safe_text
- note_file prefers the shared copy when several note rows exist

## Terms

side panel, inspector, note body, incoming edges, outgoing edges

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:80-87`
- **Signature:** `pub struct Details`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
