---
id: tests-rec
type: summary
status: unverified
title: tests::rec
source: myrn summarize
about: sym:bench/src/report.rs#report::tests::rec
anchors:
- sym:bench/src/report.rs#report::tests::rec@751e0919f7e22c88
---
# tests::rec

Test helper that builds a bugfix RunRecord with the given task, condition, run number, input-token total, and success flag. It fixes wall_ms at 1000, hook_ms at [15], and error at None.

## Connections

- **Location:** `bench/src/report.rs:573-599`
- **Signature:** `fn rec(task: &str, cond: Condition, run: usize, total: u64, success: bool) -> RunRecord`
- **In:** [[file-bench-src-report-rs]]
- **Tests:** `verdicts`
