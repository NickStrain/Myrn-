---
id: dir-bench
type: summary
status: unverified
title: bench/
source: myrn summarize
about: module:bench
anchors:
- module:bench@1bfed7a2194972b1
---
# bench/

bench/ holds the myrn-bench Rust CLI (source in src/) that measures whether the myrn context tool helps Claude Code. runner.rs runs each task repeatedly under several conditions (no myrn, injection off/on, fixed ranking) in fresh clones, metrics.rs extracts token and tool stats from stream-json sessions, and report.rs pairs the runs and applies the Gate 1 verdict. retrieval.rs checks whether `myrn context` surfaces the right files without running an agent, task.rs holds the repo and task schemas and loaders, and main.rs wires the subcommands together.

## Connections

- **In:** [[project]]
- **Directories:** [[dir-bench-src]]
