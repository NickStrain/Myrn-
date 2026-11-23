---
id: task-default-timeout
type: summary
status: unverified
title: task::default_timeout
source: myrn summarize
about: sym:bench/src/task.rs#task::default_timeout
anchors:
- sym:bench/src/task.rs#task::default_timeout@c5fe8cb7d0f7fbd9
map_version: '2'
---
# task::default_timeout

Returns the default agent session limit of 900 seconds (15 minutes). Serde calls it when timeout_secs is missing from a task.

## Behaviour

- Called with no input and always returns 900.
- Applies only to Task.timeout_secs, not Check.timeout_secs.

## Terms

default timeout, session limit, wall clock limit, 15 minutes, 900 seconds

## Connections

- **Location:** `bench/src/task.rs:71-73`
- **Signature:** `fn default_timeout() -> u64`
- **In:** [[file-bench-src-task-rs]]
