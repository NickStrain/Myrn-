---
id: file-bench-src-runner-rs
type: summary
status: unverified
title: bench/src/runner.rs
source: myrn summarize
about: file:bench/src/runner.rs
anchors:
- file:bench/src/runner.rs@a72d7e29364e519d
---
# bench/src/runner.rs

Benchmark runner for myrn. It runs each task N times per condition (without, off, with) in a fresh clone of the pinned commit with a fresh myrn home, drives Claude Code headless, then scores the run by the task check or answer match and records metrics as RunRecord. It also holds the workspace setup (prepare), the agent-free validate check, and the helpers that run commands and parse node ids.

## Connections

- **Location:** `bench/src/runner.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[runner-condition]], [[condition-as-str]], [[condition-label]], [[condition-uses-myrn]], [[condition-parse]], [[runner-runrecord]], [[runner-tools]], [[runner-paths]], [[runner-git]], [[runner-git-apply]], [[runner-prepare]], [[runner-run-check]], [[runner-run-with-timeout]], [[runner-tail]], [[runner-runspec]], [[runner-run-one]], [[runner-run-inner]], [[runner-setup-myrn]], [[runner-export-records]], [[runner-collect-injections]], [[runner-item-name]], [[runner-item-path]], [[runner-now]], [[runner-validation]], [[runner-validate]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-task-rs]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]]
