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

Parses one headless Claude Code stream-json transcript into a Session of token counts, tool calls, pre-edit exploration, shell-edit detection, and the final answer, and holds the small statistics (mean, ci95, median) the benchmark report uses. Its main parts are the Tokens and Session structs, parse(), the shell-write classifier, and the stats helpers.

## Connections

- **Location:** `bench/src/metrics.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[metrics-tokens]], [[tokens-total]], [[metrics-session]], [[session-task-tokens]], [[metrics-parse]], [[metrics-is-exploration]], [[metrics-segments]], [[metrics-is-shell-write]], [[metrics-is-write-segment]], [[metrics-redirects-to-file]], [[metrics-answer-ok]], [[metrics-mean]], [[metrics-ci95]], [[metrics-t95]], [[metrics-median]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]], [[file-bench-src-runner-rs]]
