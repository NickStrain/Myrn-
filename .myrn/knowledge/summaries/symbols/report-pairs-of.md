---
id: report-pairs-of
type: summary
status: unverified
title: report::pairs_of
source: myrn summarize
about: sym:bench/src/report.rs#report::pairs_of
anchors:
- sym:bench/src/report.rs#report::pairs_of@763bfe672f723f05
---
# report::pairs_of

Returns (a-run, b-run) pairs sharing task and run number, skipping errored runs and any run missing its counterpart, ordered by task then run number.

## Connections

- **Location:** `bench/src/report.rs:85-100`
- **Signature:** `fn pairs_of(runs: &[RunRecord], a: Condition, b: Condition) -> Vec<(&RunRecord, &RunRecord)>`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-valid]]
- **Called by:** [[report-compare-by]]
