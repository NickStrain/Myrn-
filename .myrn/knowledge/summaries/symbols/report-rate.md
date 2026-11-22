---
id: report-rate
type: summary
status: unverified
title: report::rate
source: myrn summarize
about: sym:bench/src/report.rs#report::rate
anchors:
- sym:bench/src/report.rs#report::rate@68edd1eb472fcf54
---
# report::rate

Fraction of runs with success set to true, or None for an empty slice.

## Connections

- **Location:** `bench/src/report.rs:102-108`
- **Signature:** `fn rate(xs: &[&RunRecord]) -> Option<f64>`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-compare-by]], [[report-render]]
