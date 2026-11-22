---
id: tokens-total
type: summary
status: unverified
title: Tokens::total
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Tokens::total
anchors:
- sym:bench/src/metrics.rs#metrics::Tokens::total@7932990a57c6d282
---
# Tokens::total

Returns the sum of all four token counters, including injected context and cache reads.

## Connections

- **Location:** `bench/src/metrics.rs:20-22`
- **Signature:** `pub fn total(&self) -> u64`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[real-main]], [[session-task-tokens]], [[report-total-tokens]], [[report-render]], [[runner-run-inner]]
- **Tests:** `parses_a_transcript`
