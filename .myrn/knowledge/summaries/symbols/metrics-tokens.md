---
id: metrics-tokens
type: summary
status: unverified
title: metrics::Tokens
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Tokens
anchors:
- sym:bench/src/metrics.rs#metrics::Tokens@ac5134a1a3cd5328
map_version: '2'
---
# metrics::Tokens

Holds the four API token counts for a session: input, cache creation, cache read and output.

## Behaviour

- Default -> all four counts are 0
- Serde derive: the four field names are the JSON keys

## Terms

token counts, usage, cache tokens, prompt tokens, output tokens

## Connections

- **Location:** `bench/src/metrics.rs:11-16`
- **Signature:** `pub struct Tokens`
- **In:** [[file-bench-src-metrics-rs]]
