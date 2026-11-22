---
id: report-metric
type: summary
status: unverified
title: report::Metric
source: myrn summarize
about: sym:bench/src/report.rs#report::Metric
anchors:
- sym:bench/src/report.rs#report::Metric@0463af92def3defe
---
# report::Metric

Type alias for a boxed closure returning Option<f64> per run, where None means the metric does not apply to that run.

## Connections

- **Location:** `bench/src/report.rs:71`
- **Signature:** `type Metric = Box<dyn Fn(&RunRecord) -> Option<f64>>;`
- **In:** [[file-bench-src-report-rs]]
