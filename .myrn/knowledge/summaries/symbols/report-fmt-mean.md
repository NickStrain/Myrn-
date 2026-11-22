---
id: report-fmt-mean
type: summary
status: unverified
title: report::fmt_mean
source: myrn summarize
about: sym:bench/src/report.rs#report::fmt_mean
anchors:
- sym:bench/src/report.rs#report::fmt_mean@391053e10b8182bd
---
# report::fmt_mean

Formats the mean ± 95% CI to the requested decimal digits, omitting the ± part when no CI exists and rendering '–' for empty input.

## Connections

- **Location:** `bench/src/report.rs:192-198`
- **Signature:** `fn fmt_mean(xs: &[f64], digits: usize) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]]
- **Called by:** [[report-render]]
