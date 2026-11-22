---
id: runner-runrecord
type: summary
status: unverified
title: runner::RunRecord
source: myrn summarize
about: sym:bench/src/runner.rs#runner::RunRecord
anchors:
- sym:bench/src/runner.rs#runner::RunRecord@89a7691a62175d4c
---
# runner::RunRecord

One line of results.jsonl for a single run. It holds the task and condition identity, wall time, success and timeout flags, session metrics, injected node ids and how many the agent used, hook latencies, the exported record count, check output and any error. The records field defaults to 0 when missing from older files.

## Connections

- **Location:** `bench/src/runner.rs:65-88`
- **Signature:** `pub struct RunRecord`
- **In:** [[file-bench-src-runner-rs]]
