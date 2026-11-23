---
id: task-default-check-timeout
type: summary
status: unverified
title: task::default_check_timeout
source: myrn summarize
about: sym:bench/src/task.rs#task::default_check_timeout
anchors:
- sym:bench/src/task.rs#task::default_check_timeout@8f4f0c70eebfcae9
map_version: '2'
---
# task::default_check_timeout

Returns the default check command limit of 900 seconds. Serde calls it when Check.timeout_secs is missing.

## Behaviour

- Called with no input and always returns 900.
- Independent of default_timeout, although both return 900.

## Terms

check timeout, test timeout, grader timeout, 900 seconds

## Connections

- **Location:** `bench/src/task.rs:94-96`
- **Signature:** `fn default_check_timeout() -> u64`
- **In:** [[file-bench-src-task-rs]]
