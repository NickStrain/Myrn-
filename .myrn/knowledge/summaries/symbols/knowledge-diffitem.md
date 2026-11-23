---
id: knowledge-diffitem
type: summary
status: unverified
title: knowledge::DiffItem
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::DiffItem
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::DiffItem@e8e20ffb81567cc4
map_version: '2'
---
# knowledge::DiffItem

One record in a diff, with detail describing the change, or None for added and removed records.

## Behaviour

- status differs -> detail `old -> new`
- status same but body words differ -> detail `text`
- otherwise (only links or anchors differ) -> detail `links or anchors`

## Terms

diff entry, change, changed record, status change, text change

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:90-96`
- **Signature:** `pub struct DiffItem`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
