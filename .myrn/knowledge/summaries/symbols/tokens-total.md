---
id: tokens-total
type: summary
status: unverified
title: Tokens::total
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Tokens::total
anchors:
- sym:bench/src/metrics.rs#metrics::Tokens::total@7932990a57c6d282
map_version: '2'
---
# Tokens::total

Returns the sum of all four token counts, including cached context that was re-read.

## Behaviour

- All zero -> 0
- cache_read and cache_creation are added, not excluded

## Terms

total tokens, sum, context tokens, overall usage

## Connections

- **Location:** `bench/src/metrics.rs:20-22`
- **Signature:** `pub fn total(&self) -> u64`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[real-main]], [[session-task-tokens]], [[report-total-tokens]], [[report-render]], [[runner-run-inner]]
- **Tests:** `parses_a_transcript`
