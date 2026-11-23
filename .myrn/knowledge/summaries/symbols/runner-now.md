---
id: runner-now
type: summary
status: unverified
title: runner::now
source: myrn summarize
about: sym:bench/src/runner.rs#runner::now
anchors:
- sym:bench/src/runner.rs#runner::now@c46b3c90f30dd863
map_version: '2'
---
# runner::now

Returns the current Unix time in whole seconds.

## Behaviour

- Sub-second part is dropped
- Clock before 1970 -> 0
- Returned as i64

## Terms

timestamp, epoch, unix time, clock, seconds

## Connections

- **Location:** `bench/src/runner.rs:700-705`
- **Signature:** `pub fn now() -> i64`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[real-main]], [[runner-run-one]]
