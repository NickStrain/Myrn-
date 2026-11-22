---
id: runner-run-check
type: summary
status: unverified
title: runner::run_check
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_check
anchors:
- sym:bench/src/runner.rs#runner::run_check@88d6e1167a2b81a5
---
# runner::run_check

Writes the task's hidden check files into work, then runs check.run through sh with CARGO_TARGET_DIR set. Returns whether it passed and the last 3000 bytes of output, with a timeout counted as a failure and noted in the output.

## Connections

- **Location:** `bench/src/runner.rs:205-227`
- **Signature:** `fn run_check(task: &Task, work: &Path, target: &Path) -> Result<(bool, String)>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-run-with-timeout]], [[runner-tail]]
- **Called by:** [[runner-run-inner]], [[runner-validate]]
