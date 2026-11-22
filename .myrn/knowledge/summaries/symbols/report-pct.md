---
id: report-pct
type: summary
status: unverified
title: report::pct
source: myrn summarize
about: sym:bench/src/report.rs#report::pct
anchors:
- sym:bench/src/report.rs#report::pct@cfc783129602ddac
---
# report::pct

Formats an optional ratio (0 to 1) as a whole-number percentage, rendering '–' for None. Input is a ratio, not a percent.

## Connections

- **Location:** `bench/src/report.rs:209-212`
- **Signature:** `fn pct(x: Option<f64>) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-render]]
