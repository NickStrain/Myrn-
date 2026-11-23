---
id: indexer-indexreport
type: summary
status: unverified
title: indexer::IndexReport
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport@d8931453c6957044
map_version: '2'
---
# indexer::IndexReport

Holds counts, changed paths, errors and elapsed milliseconds for one refresh, serialised to JSON with changed_paths omitted when empty.

## Behaviour

- Default -> all counts 0, edges None, lists empty
- edges stays None until resolve_all runs in refresh
- notes is filled in by the engine, not by refresh

## Terms

report, summary, stats, refresh result, counts, timing

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:26-40`
- **Signature:** `pub struct IndexReport`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
