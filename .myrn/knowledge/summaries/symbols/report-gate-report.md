---
id: report-gate-report
type: summary
status: unverified
title: report::gate
source: myrn summarize
about: sym:bench/src/report.rs#report::gate
anchors:
- sym:bench/src/report.rs#report::gate@7208c0e44b69bf6e
map_version: '2'
---
# report::gate

Gate 1 check: compares myrn with injection on (b) against no myrn (a) using compare.

## Behaviour

- Equivalent to compare(runs, Without, With)
- No complete pairs -> Verdict NoData
- A negative token change means injection used fewer tokens

## Terms

gate one, injection check, inject on, myrn gate, default decision

## Connections

- **Location:** `bench/src/report.rs:188-190`
- **Signature:** `pub fn gate(runs: &[RunRecord]) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-compare]]
- **Called by:** [[write-report]], [[report-render]]
- **Tests:** `verdicts`
