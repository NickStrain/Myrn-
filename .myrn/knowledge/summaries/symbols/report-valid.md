---
id: report-valid
type: summary
status: unverified
title: report::valid
source: myrn summarize
about: sym:bench/src/report.rs#report::valid
anchors:
- sym:bench/src/report.rs#report::valid@6e9259b7dc317d24
---
# report::valid

Returns true when the run has no error. Invalid runs are excluded from pairing and aggregation and are listed as failed in the report.

## Connections

- **Location:** `bench/src/report.rs:73-75`
- **Signature:** `fn valid(r: &RunRecord) -> bool`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-pairs-of]], [[report-render]]
