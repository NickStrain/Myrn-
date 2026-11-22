---
id: metrics-t95
type: summary
status: unverified
title: metrics::t95
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::t95
anchors:
- sym:bench/src/metrics.rs#metrics::t95@4e6d603cb5b0966a
---
# metrics::t95

Two-sided 95% Student's t critical value for the given degrees of freedom: a table up to df 30, then coarse bands, and infinity for df 0.

## Connections

- **Location:** `bench/src/metrics.rs:352-365`
- **Signature:** `fn t95(df: usize) -> f64`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[metrics-ci95]]
