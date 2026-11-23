---
id: tests-rec
type: summary
status: unverified
title: tests::rec
source: myrn summarize
about: sym:bench/src/report.rs#report::tests::rec
anchors:
- sym:bench/src/report.rs#report::tests::rec@8055d126314f5fa4
map_version: '2'
---
# tests::rec

Test fixture that builds a RunRecord from a task, condition, run number, input-token total and success flag, with other fields set to fixed defaults.

## Behaviour

- total 100000 -> session input tokens 100000, with output and cache fields 0
- Always Bugfix kind, repo 'r', wall_ms 1000, hook_ms [15], error None, timed_out false
- injected list empty and records 0; the verdicts test uses it to build paired runs

## Terms

test fixture, test helper, fake run record, mock run, sample record

## Connections

- **Location:** `bench/src/report.rs:607-635`
- **Signature:** `fn rec(task: &str, cond: Condition, run: usize, total: u64, success: bool) -> RunRecord`
- **In:** [[file-bench-src-report-rs]]
- **Tests:** `verdicts`
