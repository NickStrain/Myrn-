---
id: walks-reach
type: summary
status: unverified
title: walks::Reach
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::Reach
anchors:
- sym:crates/myrn-engine/src/retrieve/walks.rs#retrieve::walks::Reach@c261795203ee9bab
---
# walks::Reach

Result of `callers`: `callers` holds (node, distance, edge status) tuples for non-test callers, `tests` holds (test node, distance) pairs deduplicated and sorted by distance, path, and line, and `truncated` is true when the caller cap was hit and some callers were dropped.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/walks.rs:45-51`
- **Signature:** `pub struct Reach`
- **In:** [[file-crates-myrn-engine-src-retrieve-walks-rs]]
