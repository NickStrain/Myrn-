---
id: report-compare-by
type: summary
status: unverified
title: report::compare_by
source: myrn summarize
about: sym:bench/src/report.rs#report::compare_by
anchors:
- sym:bench/src/report.rs#report::compare_by@59f32d3c2846b5a3
map_version: '2'
---
# report::compare_by

Pairs runs of a and b, computes the mean paired token difference (b minus a) with a 95% interval, percent change, sign test and success rates, then assigns a Verdict and a reason.

## Behaviour

- Success of b lower than a -> Fail, checked before the token change
- Mean difference >= 0 -> Fail; otherwise an interval reaching zero, no interval, sign p >= 0.05 or no sign p -> Inconclusive
- No pairs, or a missing mean or rate -> NoData with a reason; with no interval the upper bound is infinity, so a negative mean gives Inconclusive

## Terms

paired difference, confidence interval, sign test, success check, percent change, verdict rules

## Connections

- **Location:** `bench/src/report.rs:121-185`
- **Signature:** `pub fn compare_by(runs: &[RunRecord], a: Condition, b: Condition, tokens: &TokenMeasure) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]], [[report-sign-test]], [[report-tokens]], [[report-pairs-of]], [[report-rate]], [[condition-as-str]]
- **Called by:** [[report-compare]], [[report-render]]
