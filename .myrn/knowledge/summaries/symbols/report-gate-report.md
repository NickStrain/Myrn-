---
id: report-gate-report
type: summary
status: unverified
title: report::gate
source: myrn summarize
about: sym:bench/src/report.rs#report::gate
anchors:
- sym:bench/src/report.rs#report::gate@7208c0e44b69bf6e
---
# report::gate

Gate 1 verdict: compares Condition::With (inject on) against Condition::Without on task tokens.

## Connections

- **Location:** `bench/src/report.rs:188-190`
- **Signature:** `pub fn gate(runs: &[RunRecord]) -> Gate`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[report-compare]]
- **Called by:** [[write-report]], [[report-render]]
- **Tests:** `verdicts`
