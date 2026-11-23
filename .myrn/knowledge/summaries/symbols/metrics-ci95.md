---
id: metrics-ci95
type: summary
status: unverified
title: metrics::ci95
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::ci95
anchors:
- sym:bench/src/metrics.rs#metrics::ci95@efed199e7154adb8
map_version: '2'
---
# metrics::ci95

Returns the half-width of the 95% Student-t confidence interval around the mean, using the sample variance with n-1 denominator.

## Behaviour

- [1.0, 2.0, 3.0] -> 4.303 * sqrt(1/3)
- [1.0] or [] -> None (fewer than two values)
- Returns a width to add and subtract from the mean, not the bounds themselves

## Terms

confidence interval, margin of error, error bar, standard error, t interval, uncertainty

## Connections

- **Location:** `bench/src/metrics.rs:341-349`
- **Signature:** `pub fn ci95(xs: &[f64]) -> Option<f64>`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[metrics-mean]], [[metrics-t95]]
- **Called by:** [[report-compare-by]], [[report-fmt-mean]], [[report-fmt-k]]
- **Tests:** `answers_and_stats`
