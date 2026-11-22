---
id: condition-as-str
type: summary
status: unverified
title: Condition::as_str
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Condition::as_str
anchors:
- sym:bench/src/runner.rs#runner::Condition::as_str@1cfe0eaf7a08ec46
---
# Condition::as_str

Returns the lowercase condition name used in file names and records. It is the inverse of parse.

## Connections

- **Location:** `bench/src/runner.rs:35-41`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[rescore-runs]], [[report-compare-by]], [[report-render]], [[runner-run-inner]], [[runner-export-records]]
