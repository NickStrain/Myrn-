---
id: viz-recordview
type: summary
status: unverified
title: viz::RecordView
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::RecordView
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::RecordView@9caed01cab3e8b6e
map_version: '2'
---
# viz::RecordView

A learned record flattened in, with how many times it was injected and how many times it was used.

## Behaviour

- both counts are 0 when the record has no usage rows
- fields come from RecordRow at the same JSON level

## Terms

record row, injection count, usage count, learned fact

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:115-120`
- **Signature:** `pub struct RecordView`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
