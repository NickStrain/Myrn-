---
id: report-signed-pct
type: summary
status: unverified
title: report::signed_pct
source: myrn summarize
about: sym:bench/src/report.rs#report::signed_pct
anchors:
- sym:bench/src/report.rs#report::signed_pct@e981aa5a32e73aea
---
# report::signed_pct

Formats an optional value that is already a percentage with an explicit sign and no decimals, rendering '–' for None. Unlike pct, it does not multiply by 100.

## Connections

- **Location:** `bench/src/report.rs:214-216`
- **Signature:** `fn signed_pct(x: Option<f64>) -> String`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-diff-cell]], [[report-render]]
