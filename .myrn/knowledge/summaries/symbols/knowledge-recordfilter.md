---
id: knowledge-recordfilter
type: summary
status: unverified
title: knowledge::RecordFilter
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::RecordFilter
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::RecordFilter@f83574dda3a766c4
map_version: '2'
---
# knowledge::RecordFilter

Optional type and status filters for `Engine::records`, plus a switch to include project-map summaries, which are excluded by default.

## Behaviour

- default (all None, map false) -> no type or status filtering, map summaries hidden
- ntype Some(x) -> only records whose type equals x
- status is matched against the shown (derived) status, not the written one

## Terms

filter, type filter, status filter, include map, map summaries

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:36-41`
- **Signature:** `pub struct RecordFilter`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
