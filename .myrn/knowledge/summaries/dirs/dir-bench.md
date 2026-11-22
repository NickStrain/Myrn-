---
id: dir-bench
type: summary
status: unverified
title: bench/
source: myrn summarize
about: module:bench
anchors:
- module:bench@cf28deb650143858
---
# bench/

bench/src/ builds the myrn-bench binary, a paired benchmark that runs Claude Code headless on pinned repo clones with and without myrn, then scores and reports the results. main.rs defines the clap CLI and subcommands; task.rs loads repos/*.toml and tasks/<repo>/<kind>/*.yaml; runner.rs executes N runs per condition and writes RunRecords; metrics.rs parses stream-json transcripts into Session token and tool stats; report.rs pairs runs by (task, run), computes the Gate 1 sign-test verdict, and renders report.md.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-bench-src]]
