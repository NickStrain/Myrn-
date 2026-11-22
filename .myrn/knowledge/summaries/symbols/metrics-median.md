---
id: metrics-median
type: summary
status: unverified
title: metrics::median
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::median
anchors:
- sym:bench/src/metrics.rs#metrics::median@0f578a4d7fd737cb
---
# metrics::median

Sorts the slice in place and returns the upper median (xs[len/2]), or None if empty.

## Connections

- **Location:** `bench/src/metrics.rs:367-373`
- **Signature:** `pub fn median(xs: &mut [u64]) -> Option<u64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[report-render]]
- **Tests:** `answers_and_stats`
