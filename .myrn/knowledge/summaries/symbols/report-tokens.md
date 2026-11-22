---
id: report-tokens
type: summary
status: unverified
title: report::tokens
source: myrn summarize
about: sym:bench/src/report.rs#report::tokens
anchors:
- sym:bench/src/report.rs#report::tokens@0ed0da58bf2ee095
---
# report::tokens

Gate token measure: task-only tokens via session.task_tokens(), which excludes the recording phase.

## Connections

- **Location:** `bench/src/report.rs:80-82`
- **Signature:** `fn tokens(r: &RunRecord) -> f64`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[session-task-tokens]]
- **Called by:** [[report-compare-by]], [[report-render]]
