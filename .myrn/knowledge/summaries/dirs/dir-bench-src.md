---
id: dir-bench-src
type: summary
status: unverified
title: bench/src/
source: myrn summarize
about: module:bench/src
anchors:
- module:bench/src@96bbd172f20c562b
---
# bench/src/

bench/src/ is the Rust source for the myrn-bench CLI, which measures whether the myrn context tool helps Claude Code: runner.rs executes each task repeatedly under several conditions (no myrn, injection off/on, fixed ranking) in fresh clones, metrics.rs extracts token and tool stats from stream-json sessions, and report.rs pairs the runs and applies the Gate 1 verdict. retrieval.rs checks whether `myrn context` surfaces the right files without running an agent, task.rs holds the repo and task schemas and loaders, and main.rs wires the subcommands together.

## Connections

- **In:** [[dir-bench]]
- **Files:** [[file-bench-src-main-rs]], [[file-bench-src-metrics-rs]], [[file-bench-src-report-rs]], [[file-bench-src-retrieval-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
