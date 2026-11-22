---
id: file-bench-src-report-rs
type: summary
status: unverified
title: bench/src/report.rs
source: myrn summarize
about: file:bench/src/report.rs
anchors:
- file:bench/src/report.rs@655e692c25ac0c5c
---
# bench/src/report.rs

Aggregates benchmark RunRecords, pairs runs of two conditions by (task, run number), and computes the Gate 1 verdict from paired token differences and an exact sign test. It also renders the Markdown report: gate result, paired comparison table, all-task metrics, myrn session stats, per-task table, failed runs, and a hardcoded Method section.

## Connections

- **Location:** `bench/src/report.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[report-verdict]], [[verdict-label]], [[report-gate]], [[report-sign-test]], [[report-tokenmeasure]], [[report-metric]], [[report-valid]], [[report-tokens]], [[report-pairs-of]], [[report-rate]], [[report-compare]], [[report-total-tokens]], [[report-compare-by]], [[report-gate-report]], [[report-fmt-mean]], [[report-fmt-k]], [[report-pct]], [[report-signed-pct]], [[report-diff-cell]], [[report-render]], [[tests-rec]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
- **Imported by:** [[file-bench-src-main-rs]]
