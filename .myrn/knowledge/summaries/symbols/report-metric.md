---
id: report-metric
type: summary
status: unverified
title: report::Metric
source: myrn summarize
about: sym:bench/src/report.rs#report::Metric
anchors:
- sym:bench/src/report.rs#report::Metric@0463af92def3defe
map_version: '2'
---
# report::Metric

Type alias for a boxed per-run extractor returning Option f64, used for the rows of the All tasks table.

## Behaviour

- Returns None for a run -> that run is left out of the mean for that row
- Box::new(|r| Some(tokens(r))) counts every run

## Terms

row extractor, optional value, metric row, per-run value, boxed closure

## Connections

- **Location:** `bench/src/report.rs:71`
- **Signature:** `type Metric = Box<dyn Fn(&RunRecord) -> Option<f64>>;`
- **In:** [[file-bench-src-report-rs]]
