---
id: session-task-tokens
type: summary
status: unverified
title: Session::task_tokens
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Session::task_tokens
anchors:
- sym:bench/src/metrics.rs#metrics::Session::task_tokens@9944ad5859f303d9
map_version: '2'
---
# Session::task_tokens

Returns total tokens minus the tokens spent after the Stop-hook record request, so only the task itself is counted.

## Behaviour

- record_tokens 0 -> same as tokens.total()
- record_tokens larger than total -> 0 (saturating subtraction, no underflow)

## Terms

task tokens, tokens excluding recording, recording overhead, record tokens, net tokens

## Connections

- **Location:** `bench/src/metrics.rs:68-70`
- **Signature:** `pub fn task_tokens(&self) -> u64`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[tokens-total]]
- **Called by:** [[report-tokens]]
- **Tests:** `recording_phase_is_split_off`
