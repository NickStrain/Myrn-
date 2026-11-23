---
id: report-pct
type: summary
status: unverified
title: report::pct
source: myrn summarize
about: sym:bench/src/report.rs#report::pct
anchors:
- sym:bench/src/report.rs#report::pct@cfc783129602ddac
map_version: '2'
---
# report::pct

Formats an optional success fraction as a whole-number percent, or an en dash when absent.

## Behaviour

- Some(0.5) -> '50%'
- Some(0.333) -> '33%', None -> '–'
- Multiplies by 100 before rounding, so it expects a fraction, not a percent

## Terms

success percent, fraction to percent, rounding, placeholder dash, rate display

## Connections

- **Location:** `bench/src/report.rs:209-212`
- **Signature:** `fn pct(x: Option<f64>) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-render]]
