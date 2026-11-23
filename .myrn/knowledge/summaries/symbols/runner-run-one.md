---
id: runner-run-one
type: summary
status: unverified
title: runner::run_one
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_one
anchors:
- sym:bench/src/runner.rs#runner::run_one@1c3746b500633115
map_version: '2'
---
# runner::run_one

Builds a RunRecord for the spec, runs run_inner on it, and stores any error as text in rec.error, so a record is always returned.

## Behaviour

- An error from run_inner -> success stays false and error holds the full message chain
- No error -> error stays None and success is set by run_inner
- started is taken before any work begins

## Terms

wrapper, error capture, record builder, never fails, error text, run entry point

## Connections

- **Location:** `bench/src/runner.rs:308-335`
- **Signature:** `pub fn run_one(spec: &RunSpec, tools: &Tools, paths: &Paths, raw_dir: &Path) -> RunRecord`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-run-inner]], [[runner-now]]
- **Called by:** [[real-main]]
