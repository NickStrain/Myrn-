---
id: file-bench-src-main-rs
type: summary
status: unverified
title: bench/src/main.rs
source: myrn summarize
about: file:bench/src/main.rs
anchors:
- file:bench/src/main.rs@7ff868518d90f4e7
---
# bench/src/main.rs

Entry point for the myrn-bench CLI, which runs the paired with/without benchmark (Gate 1) and its related checks. It defines the clap argument structs, dispatches each subcommand in real_main, and holds helpers for appending run records, rescoring transcripts, writing the report and resolving paths.

## Connections

- **Location:** `bench/src/main.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[cli]], [[cmd]], [[main]], [[real-main]], [[append]], [[rescore-runs]], [[write-report]], [[absolute]], [[indent]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-report-rs]], [[file-bench-src-retrieval-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
