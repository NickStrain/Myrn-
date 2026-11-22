---
id: session-task-tokens
type: summary
status: unverified
title: Session::task_tokens
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::Session::task_tokens
anchors:
- sym:bench/src/metrics.rs#metrics::Session::task_tokens@9944ad5859f303d9
---
# Session::task_tokens

Returns total tokens minus record_tokens, so the end-of-session recording phase is excluded (saturating subtraction).

## Connections

- **Location:** `bench/src/metrics.rs:68-70`
- **Signature:** `pub fn task_tokens(&self) -> u64`
- **In:** [[file-bench-src-metrics-rs]]
- **Calls:** [[tokens-total]]
- **Called by:** [[report-tokens]]
- **Tests:** `recording_phase_is_split_off`
