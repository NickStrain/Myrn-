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

Defines the serde schema for benchmark repositories (TOML files in bench/repos) and benchmark tasks (YAML files under bench/tasks), validates task consistency, and provides the loaders that read both directories.

## Connections

- **Location:** `bench/src/task.rs`
- **In:** [[dir-bench-src]]
- **Defines:** [[task-repo]], [[task-kind]], [[kind-as-str]], [[task-task]], [[task-default-timeout]], [[task-patch]], [[task-check]], [[task-default-check-timeout]], [[task-validate]], [[task-load-repos]], [[task-load-tasks]], [[task-files-with-ext]], [[task-toml-from]]
- **Imported by:** [[file-bench-src-main-rs]], [[file-bench-src-report-rs]], [[file-bench-src-retrieval-rs]], [[file-bench-src-runner-rs]]
