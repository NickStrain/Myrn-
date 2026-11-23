---
id: report-pairs-of
type: summary
status: unverified
title: report::pairs_of
source: myrn summarize
about: sym:bench/src/report.rs#report::pairs_of
anchors:
- sym:bench/src/report.rs#report::pairs_of@763bfe672f723f05
map_version: '2'
---
# report::pairs_of

Pairs each valid run of condition a with the valid run of condition b that has the same task and run number, returned in task then run order.

## Behaviour

- a equal to b -> no pairs, since every run matches a and b is never filled
- A (task, run) with only one of the two conditions is dropped
- Two runs with the same task, run and condition: the later one in the slice wins

## Terms

matched runs, paired attempts, pair matching, attempt index, run pairing

## Connections

- **Location:** `bench/src/report.rs:85-100`
- **Signature:** `fn pairs_of(runs: &[RunRecord], a: Condition, b: Condition) -> Vec<(&RunRecord, &RunRecord)>`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-valid]]
- **Called by:** [[report-compare-by]]
