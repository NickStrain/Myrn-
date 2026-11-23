---
id: runner-runrecord
type: summary
status: unverified
title: runner::RunRecord
source: myrn summarize
about: sym:bench/src/runner.rs#runner::RunRecord
anchors:
- sym:bench/src/runner.rs#runner::RunRecord@0d61231308507b8f
map_version: '2'
---
# runner::RunRecord

One line of results.jsonl describing one run: its identity, outcome, timing, agent session metrics, and myrn injection and record counts.

## Behaviour

- injected holds node ids; injected_used counts those whose file was read or edited, or whose name appears in the answer
- injected_records, injected_stale and records default to 0 when missing from older lines
- error holds the full error chain text when the run failed; check_output holds the tail of the check output

## Terms

result row, jsonl line, run result, outcome, metrics row, run log

## Connections

- **Location:** `bench/src/runner.rs:70-99`
- **Signature:** `pub struct RunRecord`
- **In:** [[file-bench-src-runner-rs]]
