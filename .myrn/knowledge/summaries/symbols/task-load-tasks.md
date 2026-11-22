---
id: task-load-tasks
type: summary
status: unverified
title: task::load_tasks
source: myrn summarize
about: sym:bench/src/task.rs#task::load_tasks
anchors:
- sym:bench/src/task.rs#task::load_tasks@1f9439dc9cdef599
---
# task::load_tasks

Walks dir recursively with an explicit stack, parses and validates each .yaml or .yml file, sorts by id, and errors on duplicate ids. Any read, parse, or validation failure aborts the whole load. It does not check that the referenced repo exists.

## Connections

- **Location:** `bench/src/task.rs:138-163`
- **Signature:** `pub fn load_tasks(dir: &Path) -> Result<Vec<Task>>`
- **In:** [[file-bench-src-task-rs]]
- **Calls:** [[task-validate]]
- **Called by:** [[real-main]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
