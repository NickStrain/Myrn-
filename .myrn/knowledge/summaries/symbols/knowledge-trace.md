---
id: knowledge-trace
type: summary
status: unverified
title: knowledge::Trace
source: myrn summarize
about: sym:crates/myrn-engine/src/knowledge.rs#knowledge::Trace
anchors:
- sym:crates/myrn-engine/src/knowledge.rs#knowledge::Trace@5eca9bf388dea604
map_version: '2'
---
# knowledge::Trace

Full provenance of one record: its body, source session, cited commits, tests, test runs, anchor states, supersede and contradict links, and Git history of its note file when shared.

## Behaviour

- private record -> note_history is empty
- no source field -> session is None
- no Git repository at the root -> commits and note_history are empty, with no error

## Terms

provenance, origin, lineage, backing evidence, where it came from, history

## Connections

- **Location:** `crates/myrn-engine/src/knowledge.rs:45-59`
- **Signature:** `pub struct Trace`
- **In:** [[file-crates-myrn-engine-src-knowledge-rs]]
