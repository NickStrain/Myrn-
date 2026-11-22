---
id: report-render
type: summary
status: unverified
title: report::render
source: myrn summarize
about: sym:bench/src/report.rs#report::render
anchors:
- sym:bench/src/report.rs#report::render@f8af9b2d2d5a40c9
---
# report::render

Builds the full Markdown report: Gate 1 verdict header, paired comparison table on task and total tokens for Without→With, Without→Off, and Off→With, all-task metrics, myrn session stats (only when a myrn condition is present), per-task table, failed-run list, and a hardcoded Method section.

## Connections

- **Location:** `bench/src/report.rs:234-566`
- **Signature:** `pub fn render(runs: &[RunRecord], title: &str) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[tokens-total]], [[metrics-mean]], [[metrics-median]], [[verdict-label]], [[report-valid]], [[report-tokens]], [[report-rate]], [[report-compare-by]], [[report-gate-report]], [[report-fmt-mean]], [[report-fmt-k]], [[report-pct]], [[report-signed-pct]], [[report-diff-cell]], [[condition-as-str]], [[condition-uses-myrn]], [[kind-as-str]]
- **Called by:** [[write-report]]
- **Tests:** `verdicts`
