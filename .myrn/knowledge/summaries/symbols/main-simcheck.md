---
id: main-simcheck
type: summary
status: unverified
title: main
source: myrn summarize
about: sym:crates/myrn-engine/examples/simcheck.rs#main
anchors:
- sym:crates/myrn-engine/examples/simcheck.rs#main@67c70eb4bee0f7d9
map_version: '2'
---
# main

For each command-line argument, embeds it, scores every row in the vectors table by dot product, sorts descending, and prints the top 6 ids with scores plus the 20th and 100th scores.

## Behaviour

- no arguments -> returns without printing anything
- rows whose id or vec column fails to decode are dropped silently via filter_map
- fewer than 100 rows -> panics at hits[99]; fewer than 20 rows panics earlier at hits[19]

## Terms

top matches, vector search, similarity scores, query embedding, dot product, ranking, debug print, smoke test

## Connections

- **Location:** `crates/myrn-engine/examples/simcheck.rs:2-23`
- **Signature:** `fn main()`
- **In:** [[file-crates-myrn-engine-examples-simcheck-rs]]
- **Calls:** [[dot]], [[from-bytes]], [[engine-open]], [[engine-db]], [[engine-embed]]
