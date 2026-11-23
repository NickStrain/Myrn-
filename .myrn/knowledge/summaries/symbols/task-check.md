---
id: task-check
type: summary
status: unverified
title: task::Check
source: myrn summarize
about: sym:bench/src/task.rs#task::Check
anchors:
- sym:bench/src/task.rs#task::Check@0184d5f0e3f8b792
map_version: '2'
---
# task::Check

Describes how an attempt is graded: hidden test files written after the session ends, a shell command whose exit status 0 means success, and a timeout. The run field is required.

## Behaviour

- Omitting files gives an empty map.
- Omitting timeout_secs gives 900.
- File keys must be relative paths without leading slash or .. components, which Task::validate enforces.

## Terms

hidden tests, grader, test command, exit status, verification, check timeout, pass fail

## Connections

- **Location:** `bench/src/task.rs:84-92`
- **Signature:** `pub struct Check`
- **In:** [[file-bench-src-task-rs]]
