---
id: runner-run-one
type: summary
status: unverified
title: runner::run_one
source: myrn summarize
about: sym:bench/src/runner.rs#runner::run_one
anchors:
- sym:bench/src/runner.rs#runner::run_one@ccdd225dd7878cdb
---
# runner::run_one

Calls run_inner and always returns a RunRecord. Any error from run_inner is stored in the record's error field, and success stays false.

## Connections

- **Location:** `bench/src/runner.rs:290-315`
- **Signature:** `pub fn run_one(spec: &RunSpec, tools: &Tools, paths: &Paths, raw_dir: &Path) -> RunRecord`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[runner-run-inner]], [[runner-now]]
- **Called by:** [[real-main]]
