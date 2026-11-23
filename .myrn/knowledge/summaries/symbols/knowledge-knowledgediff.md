---
id: knowledge-knowledgediff
type: summary
status: unverified
title: knowledge::KnowledgeDiff
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::KnowledgeDiff
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::KnowledgeDiff@c7d709db6939c220
map_version: '2'
---
# knowledge::KnowledgeDiff

Result of comparing shared notes between two points: records added, removed and changed, and a count of project-map summaries that changed.

## Behaviour

- map summaries are only counted in map_changes, never listed
- from and to are labels: a 12-character sha with the commit subject, or `working tree`
- a record present in only one snapshot is added or removed

## Terms

diff, comparison, what changed, knowledge changes, between revisions

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:79-87`
- **Signature:** `pub struct KnowledgeDiff`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
