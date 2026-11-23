---
id: file-bench-src-runner-rs
type: summary
status: unverified
title: bench/src/runner.rs
source: myrn summarize
about: file:bench/src/runner.rs
anchors:
- file:bench/src/runner.rs@f57c985a0bf4571a
---
# bench/src/runner.rs

Benchmark runner for myrn: runs each task several times per condition (without myrn, myrn with injection off, on, or fixed ranking) in a fresh clone, configures myrn and headless Claude Code for that condition, checks the result, and writes one RunRecord per run. It also builds per-repo project maps, validates tasks without an agent, and exports learned records.

## Connections

- **Location:** `bench/src/runner.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[runner-condition]], [[condition-as-str]], [[condition-label]], [[condition-uses-myrn]], [[condition-parse]], [[runner-runrecord]], [[runner-tools]], [[runner-paths]], [[runner-git]], [[runner-git-apply]], [[runner-prepare]], [[runner-run-check]], [[runner-run-with-timeout]], [[runner-tail]], [[runner-runspec]], [[runner-run-one]], [[runner-run-inner]], [[runner-setup-myrn]], [[runner-copy-dir]], [[runner-build-map]], [[runner-export-records]], [[runner-collect-injections]], [[runner-item-name]], [[runner-item-path]], [[runner-now]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-task-rs]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]], [[file-bench-src-retrieval-rs]]
