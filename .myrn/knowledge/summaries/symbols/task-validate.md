---
id: task-validate
type: summary
status: unverified
title: Task::validate
source: myrn summarize
about: sym:bench/src/task.rs#task::Task::validate
anchors:
- sym:bench/src/task.rs#task::Task::validate@80a55cdb2ba3089b
---
# Task::validate

Requires answer_contains for questions, setup and check for bugfix and replay, and check and solution for features. It also rejects check file paths that are absolute or contain '..'. It returns an error on the first failure and does not check that the repo exists or that ids are unique.

## Connections

- **Location:** `bench/src/task.rs:99-126`
- **Signature:** `pub fn validate(&self) -> Result<()>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-tasks]]
