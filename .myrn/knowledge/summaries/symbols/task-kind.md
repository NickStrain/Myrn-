---
id: task-kind
type: summary
status: unverified
title: task::Kind
source: myrn summarize
about: sym:bench/src/task.rs#task::Kind
anchors:
- sym:bench/src/task.rs#task::Kind@14c13ba9c8576b2a
map_version: '2'
---
# task::Kind

Enumerates the four task categories: Bugfix, Feature, Question, and Replay. Serialized as snake_case strings in YAML.

## Behaviour

- YAML kind value bugfix parses to Kind::Bugfix.
- An unknown kind string fails to deserialize, which stops the whole load.
- Replay is a separate kind, not an alias for Bugfix.

## Terms

task type, category, bug fix, feature task, question task, replay task

## Connections

- **Location:** `bench/src/task.rs:23-28`
- **Signature:** `pub enum Kind`
- **In:** [[file-bench-src-task-rs]]
