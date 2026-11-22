---
id: report-compare
type: summary
status: unverified
title: report::compare
source: myrn summarize
about: sym:bench/src/report.rs#report::compare
anchors:
- sym:bench/src/report.rs#report::compare@0285304bb7462bb0
---
# report::compare

Paired comparison of condition b against a on task tokens, delegating to compare_by.

## Connections

- **Location:** `bench/src/report.rs:111-113`
- **Signature:** `pub fn compare(runs: &[RunRecord], a: Condition, b: Condition) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-compare-by]]
- **Called by:** [[report-gate-report]]
