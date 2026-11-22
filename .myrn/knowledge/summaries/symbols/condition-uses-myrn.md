---
id: condition-uses-myrn
type: summary
status: unverified
title: Condition::uses_myrn
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::uses_myrn
anchors:
- sym:bench/src/runner.rs#runner::Condition::uses_myrn@8e04592adc8a61f5
---
# Condition::uses_myrn

Returns true for Off and With and false for Without. run_inner uses it to decide whether to set up myrn, collect injections and export records.

## Connections

- **Location:** `bench/src/runner.rs:50-52`
- **Signature:** `pub fn uses_myrn(self) -> bool`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[report-render]], [[runner-run-inner]]
