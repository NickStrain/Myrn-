---
id: report-signed-pct
type: summary
status: unverified
title: report::signed_pct
source: myrn summarize
about: sym:bench/src/report.rs#report::signed_pct
anchors:
- sym:bench/src/report.rs#report::signed_pct@e981aa5a32e73aea
map_version: '2'
---
# report::signed_pct

Formats an optional percent value with an explicit sign and no decimals, or an en dash when absent.

## Behaviour

- Some(-25.3) -> '-25%'
- Some(4.0) -> '+4%'
- Does not multiply by 100, so the input is already a percent, unlike pct

## Terms

signed change, percent change, explicit sign, change cell, delta display

## Connections

- **Location:** `bench/src/report.rs:214-216`
- **Signature:** `fn signed_pct(x: Option<f64>) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-diff-cell]], [[report-render]]
