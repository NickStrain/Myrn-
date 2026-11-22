---
id: metrics-ci95
type: summary
status: unverified
title: metrics::ci95
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::ci95
anchors:
- sym:bench/src/metrics.rs#metrics::ci95@efed199e7154adb8
---
# metrics::ci95

Half-width of the 95% confidence interval of the mean using Student's t; returns None when fewer than two values.

## Connections

- **Location:** `bench/src/metrics.rs:341-349`
- **Signature:** `pub fn ci95(xs: &[f64]) -> Option<f64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-mean]], [[metrics-t95]]
- **Called by:** [[report-compare-by]], [[report-fmt-mean]], [[report-fmt-k]]
- **Tests:** `answers_and_stats`
