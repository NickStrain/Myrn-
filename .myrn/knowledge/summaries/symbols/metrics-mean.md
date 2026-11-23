---
id: metrics-mean
type: summary
status: unverified
title: metrics::mean
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::mean
anchors:
- sym:bench/src/metrics.rs#metrics::mean@e53d2c2264312973
map_version: '2'
---
# metrics::mean

Returns the arithmetic mean of a slice of f64.

## Behaviour

- [1.0, 2.0, 3.0] -> Some(2.0)
- [] -> None
- [5.0] -> Some(5.0)

## Terms

average, arithmetic mean, sample average

## Connections

- **Location:** `bench/src/metrics.rs:332-338`
- **Signature:** `pub fn mean(xs: &[f64]) -> Option<f64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-ci95]], [[report-compare-by]], [[report-fmt-mean]], [[report-fmt-k]], [[report-render]]
- **Tests:** `answers_and_stats`
