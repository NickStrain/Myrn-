---
id: task-task
type: summary
status: unverified
title: task::Task
source: myrn summarize
about: sym:bench/src/task.rs#task::Task
anchors:
- sym:bench/src/task.rs#task::Task@420ee4fd541c43c8
map_version: '2'
---
# task::Task

Holds one benchmark task: id, repo, kind, the prompt the agent sees, and optional setup, solution, and check sections. Unknown YAML keys are rejected, and the struct fills in defaults for omitted optional fields.

## Behaviour

- A YAML key not declared on the struct fails parsing because of deny_unknown_fields.
- Omitting timeout_secs gives 900.
- Omitting answer_contains, touches, setup, solution, or check gives an empty list or None.

## Terms

benchmark task, yaml task, task definition, agent session, session timeout, touches, answer_contains, prompt

## Connections

- **Location:** `bench/src/task.rs:43-69`
- **Signature:** `pub struct Task`
- **In:** [[file-bench-src-task-rs]]
