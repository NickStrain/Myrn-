---
id: metrics-median
type: summary
status: unverified
title: metrics::median
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::median
anchors:
- sym:bench/src/metrics.rs#metrics::median@0f578a4d7fd737cb
map_version: '2'
---
# metrics::median

Sorts the given slice in place and returns the upper middle element, or None if empty.

## Behaviour

- [5, 1, 3] -> Some(3), and the slice is left as [1, 3, 5]
- [1, 2, 3, 4] -> Some(3), the upper middle, not an average
- [] -> None

## Terms

middle value, median, sorted middle, in-place sort, robust centre

## Connections

- **Location:** `bench/src/metrics.rs:367-373`
- **Signature:** `pub fn median(xs: &mut [u64]) -> Option<u64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[report-render]]
- **Tests:** `answers_and_stats`
