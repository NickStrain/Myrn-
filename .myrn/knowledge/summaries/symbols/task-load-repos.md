---
id: task-load-repos
type: summary
status: unverified
title: task::load_repos
source: myrn summarize
about: sym:bench/src/task.rs#task::load_repos
anchors:
- sym:bench/src/task.rs#task::load_repos@33ced75e8b156aff
---
# task::load_repos

Reads every .toml file directly in dir (not recursive) into a BTreeMap keyed by repo name. A duplicate name silently overwrites the earlier entry.

## Connections

- **Location:** `bench/src/task.rs:129-136`
- **Signature:** `pub fn load_repos(dir: &Path) -> Result<BTreeMap<String, Repo>>`
- **In:** [[file-bench-src-task-rs]]
- **Calls:** [[task-files-with-ext]], [[task-toml-from]]
- **Called by:** [[real-main]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
