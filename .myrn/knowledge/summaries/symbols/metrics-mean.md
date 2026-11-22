---
id: metrics-mean
type: summary
status: unverified
title: metrics::mean
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::mean
anchors:
- sym:bench/src/metrics.rs#metrics::mean@e53d2c2264312973
---
# metrics::mean

Arithmetic mean of a slice of f64; returns None for an empty slice.

## Connections

- **Location:** `bench/src/metrics.rs:332-338`
- **Signature:** `pub fn mean(xs: &[f64]) -> Option<f64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-ci95]], [[report-compare-by]], [[report-fmt-mean]], [[report-fmt-k]], [[report-render]]
- **Tests:** `answers_and_stats`
