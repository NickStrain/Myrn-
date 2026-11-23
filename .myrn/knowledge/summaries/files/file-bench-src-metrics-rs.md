---
id: file-bench-src-metrics-rs
type: summary
status: unverified
title: bench/src/metrics.rs
source: myrn summarize
about: file:bench/src/metrics.rs
anchors:
- file:bench/src/metrics.rs@b477b434d00ae520
---
# bench/src/metrics.rs

Computes metrics for one headless Claude Code stream-json session: token totals, tool and exploration counts, shell-edit detection, the split-off Stop-hook recording phase, and the final answer. It also has the mean, 95% confidence interval and median helpers the report uses.

## Connections

- **Location:** `bench/src/metrics.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[metrics-tokens]], [[tokens-total]], [[metrics-session]], [[session-task-tokens]], [[metrics-parse]], [[metrics-is-exploration]], [[metrics-segments]], [[metrics-is-shell-write]], [[metrics-is-write-segment]], [[metrics-redirects-to-file]], [[metrics-answer-ok]], [[metrics-mean]], [[metrics-ci95]], [[metrics-t95]], [[metrics-median]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]], [[file-bench-src-runner-rs]]
