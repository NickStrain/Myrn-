---
id: task-patch
type: summary
status: unverified
title: task::Patch
source: myrn summarize
about: sym:bench/src/task.rs#task::Patch
anchors:
- sym:bench/src/task.rs#task::Patch@811a9789d8a24d38
map_version: '2'
---
# task::Patch

Wraps a unified diff string that is applied to the repo with git apply. Used for setup and solution.

## Behaviour

- Any key other than patch fails parsing because of deny_unknown_fields.
- The diff text is not checked here; a malformed diff fails only when git apply runs.

## Terms

diff, unified diff, git apply, setup patch, solution patch, reference fix

## Connections

- **Location:** `bench/src/task.rs:77-80`
- **Signature:** `pub struct Patch`
- **In:** [[file-bench-src-task-rs]]
