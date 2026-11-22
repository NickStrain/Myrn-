---
id: rescore-runs
type: summary
status: unverified
title: rescore_runs
source: myrn summarize
about: sym:bench/src/main.rs#rescore_runs
anchors:
- sym:bench/src/main.rs#rescore_runs@f836308bc8b5c16d
---
# rescore_runs

Reads results.jsonl in the run dir and reparses each record's raw/<task>--<condition>--<run>.jsonl transcript with metrics::parse, copying tool-use metrics into the record while keeping tokens, success and injection data; it cuts the transcript at the stderr separator and rewrites results.jsonl in place. Records with no matching raw file or task are written back unchanged.

## Connections

- **Location:** `bench/src/main.rs:381-422`
- **Signature:** `fn rescore_runs(dir: &Path, tasks: &[task::Task]) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[metrics-parse]], [[condition-as-str]]
- **Called by:** [[real-main]]
