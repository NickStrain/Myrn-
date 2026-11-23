---
id: symbols-edgecounts
type: summary
status: unverified
title: symbols::EdgeCounts
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::EdgeCounts
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::EdgeCounts@9eb68a08d7727e7f
map_version: '2'
---
# symbols::EdgeCounts

Holds counts of edges written in one resolution pass: calls, inferred calls, imports and tests.

## Behaviour

- default -> all counts zero
- calls_inferred counts only calls that were inferred and newly inserted
- an edge already present is not counted again

## Terms

edge counts, resolution stats, edge statistics, counters, summary numbers

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:935-940`
- **Signature:** `pub struct EdgeCounts`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
