---
id: report-compare-by
type: summary
status: unverified
title: report::compare_by
source: myrn summarize
about: sym:bench/src/report.rs#report::compare_by
anchors:
- sym:bench/src/report.rs#report::compare_by@59f32d3c2846b5a3
---
# report::compare_by

Computes the paired diff (b minus a) on the given token measure, with mean, 95% CI, percent change, and a sign test over cheaper pairs. Verdict is NoData with no complete pairs, Fail if success drops or the mean diff is >= 0, Inconclusive if the CI includes zero or the sign-test p >= 0.05, and Pass otherwise.

## Connections

- **Location:** `bench/src/report.rs:121-185`
- **Signature:** `pub fn compare_by(runs: &[RunRecord], a: Condition, b: Condition, tokens: &TokenMeasure) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]], [[report-sign-test]], [[report-tokens]], [[report-pairs-of]], [[report-rate]], [[condition-as-str]]
- **Called by:** [[report-compare]], [[report-render]]
