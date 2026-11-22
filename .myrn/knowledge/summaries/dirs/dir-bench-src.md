---
id: dir-bench-src
type: summary
status: unverified
title: bench/src/
source: myrn summarize
about: module:bench/src
anchors:
- module:bench/src@12ccb3121be7563f
---
# bench/src/

bench/src/ holds the myrn-bench binary, a paired benchmark that runs Claude Code headless on pinned repo clones with and without myrn, then scores and reports the results. main.rs wires the clap CLI and subcommands; task.rs loads repos/*.toml and tasks/<repo>/<kind>/*.yaml; runner.rs executes N runs per condition and writes RunRecords; metrics.rs parses stream-json transcripts into Session token and tool stats; report.rs pairs runs by (task, run), computes the Gate 1 sign-test verdict, and renders report.md.

## Connections

- **In:** [[dir-bench]]
- **Files:** [[file-bench-src-main-rs]], [[file-bench-src-metrics-rs]], [[file-bench-src-report-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
