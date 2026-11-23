---
id: walks-reach
type: summary
status: unverified
title: walks::Reach
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::Reach
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::Reach@c261795203ee9bab
map_version: '2'
---
# walks::Reach

Result of callers: non-test callers with their BFS distance and optional edge status, test nodes with distance, and a flag set when callers were dropped at the cap.

## Behaviour

- callers entries are (node, distance, status); distance 1 means a direct caller
- tests entries are (node, distance), sorted by distance, then path, then start_line
- truncated is true only when non-test callers were skipped because the cap was reached

## Terms

impact result, blast radius, traversal result, reach set, truncation flag, caller list, affected tests

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:45-51`
- **Signature:** `pub struct Reach`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
