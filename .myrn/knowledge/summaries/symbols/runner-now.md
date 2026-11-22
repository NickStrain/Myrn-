---
id: runner-now
type: summary
status: unverified
title: runner::now
source: myrn summarize
about: sym:bench/src/runner.rs#runner::now
anchors:
- sym:bench/src/runner.rs#runner::now@c46b3c90f30dd863
---
# runner::now

Returns the current Unix time in whole seconds, or 0 if the clock is before the epoch.

## Connections

- **Location:** `bench/src/runner.rs:545-550`
- **Signature:** `pub fn now() -> i64`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[real-main]], [[runner-run-one]]
