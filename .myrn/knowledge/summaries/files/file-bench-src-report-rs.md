---
id: file-bench-src-report-rs
type: summary
status: unverified
title: bench/src/report.rs
source: myrn summarize
about: file:bench/src/report.rs
anchors:
- file:bench/src/report.rs@de9a84e1a6761e81
---
# bench/src/report.rs

Aggregates benchmark run records into paired comparisons: it pairs runs by task and attempt, computes mean token differences with 95% intervals and a sign test, assigns the Gate 1 verdict, and renders the Markdown report. Main parts are the statistics helpers, compare_by and gate, and render with its small formatting helpers.

## Connections

- **Location:** `bench/src/report.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[report-verdict]], [[verdict-label]], [[report-gate]], [[report-sign-test]], [[report-tokenmeasure]], [[report-metric]], [[report-valid]], [[report-tokens]], [[report-pairs-of]], [[report-rate]], [[report-compare]], [[report-total-tokens]], [[report-compare-by]], [[report-gate-report]], [[report-fmt-mean]], [[report-fmt-k]], [[report-pct]], [[report-signed-pct]], [[report-diff-cell]], [[report-render]], [[tests-rec]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
- **Imported by:** [[file-bench-src-main-rs]]
