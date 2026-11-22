---
id: task-task
type: summary
status: unverified
title: task::Task
source: myrn summarize
about: sym:bench/src/task.rs#task::Task
anchors:
- sym:bench/src/task.rs#task::Task@420ee4fd541c43c8
---
# task::Task

One task parsed from YAML with deny_unknown_fields. id, repo, kind, and prompt are required. setup, solution, check, answer_contains, and touches are optional or defaulted, and timeout_secs defaults to 900. The prompt should describe the symptom, not the location.

## Connections

- **Location:** `bench/src/task.rs:43-69`
- **Signature:** `pub struct Task`
- **In:** [[file-bench-src-task-rs]]
