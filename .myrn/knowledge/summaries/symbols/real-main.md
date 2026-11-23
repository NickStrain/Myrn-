---
id: real-main
type: summary
status: unverified
title: real_main
source: myrn summarize
about: sym:bench/src/main.rs#real_main
anchors:
- sym:bench/src/main.rs#real_main@192ed356bcc39b49
map_version: '2'
---
# real_main

Parses arguments, loads repos and tasks, then runs the selected subcommand and returns its exit code. Run, Fetch, Retrieval and Review write files, and Run and Retrieval start agent or myrn work.

## Behaviour

- Validate: any task that fails its check -> FAILURE, otherwise SUCCESS
- Run --dry-run -> prints the plan and returns SUCCESS without running any agent
- Run with no --myrn and no ../target/release/myrn -> error asking for a build

## Terms

dispatch, command handling, load tasks, main logic, run benchmark, task selection, session plan

## Connections

- **Location:** `bench/src/main.rs:142-506`
- **Signature:** `fn real_main() -> Result<ExitCode>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[append]], [[rescore-runs]], [[write-report]], [[absolute]], [[indent]], [[tokens-total]], [[retrieval-one]], [[retrieval-render]], [[condition-uses-myrn]], [[condition-parse]], [[runner-run-one]], [[runner-build-map]], [[runner-now]], [[runner-validate]], [[task-load-repos]], [[task-load-tasks]], [[model-dir]], [[review-sample]], [[review-checklist]], [[review-score-review]], [[paths-data-home]]
- **Called by:** [[main]]
