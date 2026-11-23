---
id: runner-run-with-timeout
type: summary
status: unverified
title: runner::run_with_timeout
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_with_timeout
anchors:
- sym:bench/src/runner.rs#runner::run_with_timeout@c1801f5201287b0f
map_version: '2'
---
# runner::run_with_timeout

Starts a command, reads stdout and stderr on separate threads, kills it after the limit, and returns the exit code, the output, and whether it timed out.

## Behaviour

- Non-empty stderr is appended after the marker "--- stderr ---"
- Killed by timeout -> timed_out true; the exit code is None when a signal ended the process
- Polls every 200 ms, so the limit is approximate

## Terms

timeout, kill, subprocess, capture output, watchdog, deadline, time limit

## Connections

- **Location:** `bench/src/runner.rs:248-286`
- **Signature:** `fn run_with_timeout(mut cmd: Command, limit: Duration) -> Result<(Option<i32>, String, bool)>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-run-check]], [[runner-run-inner]]
