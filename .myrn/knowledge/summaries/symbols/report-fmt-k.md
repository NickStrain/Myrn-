---
id: report-fmt-k
type: summary
status: unverified
title: report::fmt_k
source: myrn summarize
about: sym:bench/src/report.rs#report::fmt_k
anchors:
- sym:bench/src/report.rs#report::fmt_k@de3908dc9cda5c6c
---
# report::fmt_k

Divides values by 1000 and formats the mean ± CI as one-decimal 'k' figures, rendering '–' for empty input.

## Connections

- **Location:** `bench/src/report.rs:200-207`
- **Signature:** `fn fmt_k(xs: &[f64]) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]]
- **Called by:** [[report-render]]
