---
id: runner-run-check
type: summary
status: unverified
title: runner::run_check
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_check
anchors:
- sym:bench/src/runner.rs#runner::run_check@88d6e1167a2b81a5
map_version: '2'
---
# runner::run_check

Writes the hidden check files into the workspace, runs the task check command with CARGO_TARGET_DIR set, and returns a pass flag with the tail of the output.

## Behaviour

- No check -> (true, empty string)
- Exit status 0 and not timed out -> true; timeout -> false, with "[check timed out]" appended
- Output is the last 3000 bytes

## Terms

test, hidden tests, verify, grader, pass or fail, test runner, evaluation, check command

## Connections

- **Location:** `bench/src/runner.rs:223-245`
- **Signature:** `fn run_check(task: &Task, work: &Path, target: &Path) -> Result<(bool, String)>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-run-with-timeout]], [[runner-tail]]
- **Called by:** [[runner-run-inner]], [[runner-validate]]
