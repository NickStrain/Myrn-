---
id: file-bench-src-main-rs
type: summary
status: unverified
title: bench/src/main.rs
source: myrn summarize
about: file:bench/src/main.rs
anchors:
- file:bench/src/main.rs@ffb135a00921eb46
---
# bench/src/main.rs

Entry point for the myrn-bench binary, a paired with/without-myrn benchmark driver that lists and fetches repos, validates tasks, runs agent sessions, writes results.jsonl and report.md, and creates review checklists. Main parts are the clap CLI definitions (Cli, Cmd), the subcommand dispatch in real_main, and the helpers that append records, rescore saved transcripts, and render the report.

## Connections

- **Location:** `bench/src/main.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[cli]], [[cmd]], [[main]], [[real-main]], [[append]], [[rescore-runs]], [[write-report]], [[absolute]], [[indent]]
- **Imports:** [[file-bench-src-metrics-rs]], [[file-bench-src-report-rs]], [[file-bench-src-runner-rs]], [[file-bench-src-task-rs]]
