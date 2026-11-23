---
id: runner-export-records
type: summary
status: unverified
title: runner::export_records
source: myrn summarize
about: sym:bench/src/runner.rs#runner::export_records
anchors:
- sym:bench/src/runner.rs#runner::export_records@16c661c7175b7a25
map_version: '2'
---
# runner::export_records

Runs myrn debug review in the workspace and saves the returned review items as JSON in the records directory, each tagged with its task, condition and run.

## Behaviour

- No items -> returns 0 and writes no file
- Command failure -> error with stderr
- File name replaces / with _ in the task id; returns the item count

## Terms

review export, learned notes, gate 2, save records, dump, origin tag, record count

## Connections

- **Location:** `bench/src/runner.rs:605-646`
- **Signature:** `fn export_records( myrn: &Path, work: &Path, home: &Path, dir: &Path, spec: &RunSpec, ) -> Result<usize>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[condition-as-str]]
- **Called by:** [[runner-run-inner]]
