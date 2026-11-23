---
id: report-render
type: summary
status: unverified
title: report::render
source: myrn summarize
about: sym:bench/src/report.rs#report::render
anchors:
- sym:bench/src/report.rs#report::render@69a0ecf4c1bb5a6f
map_version: '2'
---
# report::render

Builds the full Markdown report string from runs: Gate 1 verdict and reason, paired comparison table, all-tasks metrics, myrn sessions table, per-task table, failed runs list and a fixed method section.

## Behaviour

- No runs -> Gate 1 shows NO DATA with the no complete pairs reason
- The Step 20 section appears only when Fixed and With have pairs, and the myrn table only when a myrn condition is present
- Condition columns follow the fixed order Without, Off, Fixed, With, limited to conditions present in runs

## Terms

markdown report, summary, gate section, comparison table, per-task table, method text, report output

## Connections

- **Location:** `bench/src/report.rs:239-600`
- **Signature:** `pub fn render(runs: &[RunRecord], title: &str) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[tokens-total]], [[metrics-mean]], [[metrics-median]], [[verdict-label]], [[report-valid]], [[report-tokens]], [[report-rate]], [[report-compare]], [[report-compare-by]], [[report-gate-report]], [[report-fmt-mean]], [[report-fmt-k]], [[report-pct]], [[report-signed-pct]], [[report-diff-cell]], [[condition-as-str]], [[condition-uses-myrn]], [[kind-as-str]]
- **Called by:** [[write-report]]
- **Tests:** `verdicts`
