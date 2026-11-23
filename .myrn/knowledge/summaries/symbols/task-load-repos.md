---
id: task-load-repos
type: summary
status: unverified
title: task::load_repos
source: myrn summarize
about: sym:bench/src/task.rs#task::load_repos
anchors:
- sym:bench/src/task.rs#task::load_repos@33ced75e8b156aff
map_version: '2'
---
# task::load_repos

Reads every .toml file directly inside dir (not recursive), parses each as a Repo, and returns a map keyed by repo name. Files are processed in sorted path order.

## Behaviour

- Two files with the same name field: the later file in sort order silently replaces the earlier one, with no error.
- An empty directory returns an empty map.
- Files without a .toml extension are ignored.

## Terms

repo loader, repos directory, toml files, repo map, repository list, name key

## Connections

- **Location:** `bench/src/task.rs:129-136`
- **Signature:** `pub fn load_repos(dir: &Path) -> Result<BTreeMap<String, Repo>>`
- **In:** [[file-bench-src-task-rs]]
- **Calls:** [[task-files-with-ext]], [[task-toml-from]]
- **Called by:** [[real-main]]
- **Tests:** `the_shipped_task_set_parses_and_is_consistent`
