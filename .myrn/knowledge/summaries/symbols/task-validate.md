---
id: task-validate
type: summary
status: unverified
title: Task::validate
source: myrn summarize
about: sym:bench/src/task.rs#task::Task::validate
anchors:
- sym:bench/src/task.rs#task::Task::validate@80a55cdb2ba3089b
map_version: '2'
---
# Task::validate

Checks that a task has the fields its kind requires and that every check file path stays inside the repo. Returns an error naming the task id on the first failure.

## Behaviour

- A Question with an empty answer_contains returns an error.
- A Bugfix or Replay without both setup and check returns an error.
- A check file key of /etc/x or a/../b is rejected; a/b..c is allowed because only a leading slash and an exact .. component are rejected.

## Terms

validation, schema check, path traversal, relative path, consistency check, required fields, bail

## Connections

- **Location:** `bench/src/task.rs:99-126`
- **Signature:** `pub fn validate(&self) -> Result<()>`
- **In:** [[file-bench-src-task-rs]]
- **Called by:** [[task-load-tasks]]
