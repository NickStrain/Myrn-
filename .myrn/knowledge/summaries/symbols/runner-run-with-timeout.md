---
id: runner-run-with-timeout
type: summary
status: unverified
title: runner::run_with_timeout
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_with_timeout
anchors:
- sym:bench/src/runner.rs#runner::run_with_timeout@c1801f5201287b0f
---
# runner::run_with_timeout

Runs a command with piped stdout and stderr and kills it after the limit. Returns the exit code (None if killed by a signal), the output with stderr appended after a marker line, and whether it timed out.

## Connections

- **Location:** `bench/src/runner.rs:230-268`
- **Signature:** `fn run_with_timeout(mut cmd: Command, limit: Duration) -> Result<(Option<i32>, String, bool)>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-run-check]], [[runner-run-inner]]
