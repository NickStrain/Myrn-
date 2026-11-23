---
id: runner-runspec
type: summary
status: unverified
title: runner::RunSpec
source: myrn summarize
about: sym:bench/src/runner.rs#runner::RunSpec
anchors:
- sym:bench/src/runner.rs#runner::RunSpec@454f34d423b046ca
map_version: '2'
---
# runner::RunSpec

Borrowed inputs for one run: the task, its repo, the condition, and the run number.

## Behaviour

- Holds references, so the task and repo are borrowed, not owned
- All four fields are required; there is no default
- Passed to run_one and run_inner

## Terms

run input, run arguments, run descriptor, borrowed parameters, run parameters

## Connections

- **Location:** `bench/src/runner.rs:301-306`
- **Signature:** `pub struct RunSpec<'a>`
- **In:** [[file-bench-src-runner-rs]]
