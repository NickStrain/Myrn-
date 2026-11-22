---
id: file-bench-src-task-rs
type: summary
status: unverified
title: bench/src/task.rs
source: myrn summarize
about: file:bench/src/task.rs
anchors:
- file:bench/src/task.rs@e3924f58455ce264
---
# bench/src/task.rs

Loads benchmark repo definitions from bench/repos/*.toml and task definitions from bench/tasks/<repo>/<kind>/*.yaml into typed structs, validates tasks by kind, and includes a test that checks the shipped task set. Main parts are Repo, Kind, Task, Patch, Check, and the loaders.

## Connections

- **Location:** `bench/src/task.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[task-repo]], [[task-kind]], [[kind-as-str]], [[task-task]], [[task-default-timeout]], [[task-patch]], [[task-check]], [[task-default-check-timeout]], [[task-validate]], [[task-load-repos]], [[task-load-tasks]], [[task-files-with-ext]], [[task-toml-from]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]], [[file-bench-src-runner-rs]]
