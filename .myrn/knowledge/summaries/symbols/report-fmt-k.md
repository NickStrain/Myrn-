---
id: report-fmt-k
type: summary
status: unverified
title: report::fmt_k
source: myrn summarize
about: sym:bench/src/report.rs#report::fmt_k
anchors:
- sym:bench/src/report.rs#report::fmt_k@de3908dc9cda5c6c
map_version: '2'
---
# report::fmt_k

Formats a slice of token counts in thousands, as mean ± interval with one decimal and a 'k' suffix.

## Behaviour

- [] -> the en dash placeholder
- Values are divided by 1000 before the mean and interval are computed
- No interval available -> 'Nk' with no ± part

## Terms

thousands, k suffix, token count cell, kilo formatting, token display

## Connections

- **Location:** `bench/src/report.rs:200-207`
- **Signature:** `fn fmt_k(xs: &[f64]) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[metrics-mean]], [[metrics-ci95]]
- **Called by:** [[report-render]]
