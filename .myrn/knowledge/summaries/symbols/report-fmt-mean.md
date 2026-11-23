---
id: report-fmt-mean
type: summary
status: unverified
title: report::fmt_mean
source: myrn summarize
about: sym:bench/src/report.rs#report::fmt_mean
anchors:
- sym:bench/src/report.rs#report::fmt_mean@391053e10b8182bd
map_version: '2'
---
# report::fmt_mean

Formats the mean and 95% interval of a slice as 'mean ± interval' with a fixed number of decimal places.

## Behaviour

- [] -> the en dash placeholder
- No interval available -> mean alone, without the ± part
- digits sets the decimal places for both mean and interval

## Terms

mean with interval, decimal places, en dash placeholder, stat cell, formatted statistic

## Connections

- **Location:** `bench/src/report.rs:192-198`
- **Signature:** `fn fmt_mean(xs: &[f64], digits: usize) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]]
- **Called by:** [[report-render]]
