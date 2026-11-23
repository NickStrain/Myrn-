---
id: knowledge-recordrow
type: summary
status: unverified
title: knowledge::RecordRow
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::RecordRow
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::RecordRow@d2d29c2b52c2d63b
map_version: '2'
---
# knowledge::RecordRow

One record as listed by `log` and shown in traces, with its derived status, the status written in the note, title, visibility, source, file and time.

## Behaviour

- status is the graph node status when one exists, otherwise the written status
- written_status is always the status stored in the note itself
- time is the note file mtime converted from nanoseconds to unix seconds

## Terms

log entry, list item, derived status, written status, record summary

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:21-33`
- **Signature:** `pub struct RecordRow`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
