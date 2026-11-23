---
id: task-repo
type: summary
status: unverified
title: task::Repo
source: myrn summarize
about: sym:bench/src/task.rs#task::Repo
anchors:
- sym:bench/src/task.rs#task::Repo@ad1a25bd09eb3d04
map_version: '2'
---
# task::Repo

Holds one benchmark repository: its name, clone URL, the full commit id each run starts from, its language, and an optional size label. Deserialized from TOML.

## Behaviour

- A TOML file without a size key gives size None.
- The name field becomes the key in the map returned by load_repos.
- The commit field must be a full commit id; the struct does not check its format.

## Terms

repository, repo definition, commit pin, starting commit, clone url, benchmark repo, toml config

## Connections

- **Location:** `bench/src/task.rs:11-19`
- **Signature:** `pub struct Repo`
- **In:** [[file-bench-src-task-rs]]
