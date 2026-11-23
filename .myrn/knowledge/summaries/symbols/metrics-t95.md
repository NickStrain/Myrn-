---
id: metrics-t95
type: summary
status: unverified
title: metrics::t95
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::t95
anchors:
- sym:bench/src/metrics.rs#metrics::t95@4e6d603cb5b0966a
map_version: '2'
---
# metrics::t95

Returns the two-sided 95% Student-t critical value for the given degrees of freedom from a lookup table.

## Behaviour

- df 1 -> 12.706; df 30 -> 2.042
- df 31 to 60 -> 2.000; df 61 to 120 -> 1.980; above 120 -> 1.960
- df 0 -> infinity

## Terms

t critical value, t distribution, critical value, degrees of freedom, significance factor

## Connections

- **Location:** `bench/src/metrics.rs:352-365`
- **Signature:** `fn t95(df: usize) -> f64`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-ci95]]
