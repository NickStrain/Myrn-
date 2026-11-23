---
id: task-load-tasks
type: summary
status: unverified
title: task::load_tasks
source: myrn summarize
about: sym:bench/src/task.rs#task::load_tasks
anchors:
- sym:bench/src/task.rs#task::load_tasks@1f9439dc9cdef599
map_version: '2'
---
# task::load_tasks

Walks dir recursively, parses each .yaml or .yml file as a Task, validates it, and returns all tasks sorted by id. Fails if two tasks share an id.

## Behaviour

- Two YAML files with the same id return a duplicate task id error.
- A single file that fails to parse or validate aborts the whole load.
- load_tasks does not check that task.repo exists in the repo map; the shipped-task test does that.

## Terms

task loader, recursive scan, yaml files, duplicate id, sort order, task set, directory walk

## Connections

- **Location:** `bench/src/task.rs:138-163`
- **Signature:** `pub fn load_tasks(dir: &Path) -> Result<Vec<Task>>`
- **In:** [[file-bench-src-task-rs]]
- **Calls:** [[task-validate]]
- **Called by:** [[real-main]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
