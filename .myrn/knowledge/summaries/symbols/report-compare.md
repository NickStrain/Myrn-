---
id: report-compare
type: summary
status: unverified
title: report::compare
source: myrn summarize
about: sym:bench/src/report.rs#report::compare
anchors:
- sym:bench/src/report.rs#report::compare@0285304bb7462bb0
map_version: '2'
---
# report::compare

Compares condition b against condition a on task tokens by calling compare_by with the tokens measure.

## Behaviour

- compare(runs, Without, With) -> Gate on task tokens
- Returns the same Gate that compare_by gives for the tokens measure
- render uses it for the Fixed against With comparison

## Terms

paired comparison, task token comparison, condition pair, a versus b

## Connections

- **Location:** `bench/src/report.rs:111-113`
- **Signature:** `pub fn compare(runs: &[RunRecord], a: Condition, b: Condition) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-compare-by]]
- **Called by:** [[report-gate-report]], [[report-render]]
