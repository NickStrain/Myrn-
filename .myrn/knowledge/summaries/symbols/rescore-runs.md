---
id: rescore-runs
type: summary
status: unverified
title: rescore_runs
source: myrn summarize
about: sym:bench/src/main.rs#rescore_runs
anchors:
- sym:bench/src/main.rs#rescore_runs@f836308bc8b5c16d
map_version: '2'
---
# rescore_runs

Rewrites results.jsonl in place, recomputing tool-use metrics from each run's saved raw transcript while keeping tokens, success and injection data.

## Behaviour

- raw transcript and matching task found -> exploration, edits, tool-call and record fields replaced by the new parse
- raw transcript or task missing -> record written back unchanged
- answer replaced only when the new parse reports asked_to_record

## Terms

rescore, recompute metrics, re-parse transcript, fix metrics, tool metrics, reprocess runs, raw transcript

## Connections

- **Location:** `bench/src/main.rs:519-560`
- **Signature:** `fn rescore_runs(dir: &Path, tasks: &[task::Task]) -> Result<()>`
- **In:** [[file-bench-src-main-rs]]
- **Calls:** [[metrics-parse]], [[condition-as-str]]
- **Called by:** [[real-main]]
