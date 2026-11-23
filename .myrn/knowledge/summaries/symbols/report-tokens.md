---
id: report-tokens
type: summary
status: unverified
title: report::tokens
source: myrn summarize
about: sym:bench/src/report.rs#report::tokens
anchors:
- sym:bench/src/report.rs#report::tokens@0ed0da58bf2ee095
map_version: '2'
---
# report::tokens

Returns the per-run task token count from session.task_tokens(), excluding the recording phase; it is the default measure for Gate 1.

## Behaviour

- Returns task_tokens() as f64
- Recording-phase tokens are left out, so the gate measures the task alone
- Used as the default measure in compare and the per-task rows

## Terms

task tokens, task cost, recording excluded, default measure, token count

## Connections

- **Location:** `bench/src/report.rs:80-82`
- **Signature:** `fn tokens(r: &RunRecord) -> f64`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[session-task-tokens]]
- **Called by:** [[report-compare-by]], [[report-render]]
