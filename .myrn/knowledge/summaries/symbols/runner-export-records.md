---
id: runner-export-records
type: summary
status: unverified
title: runner::export_records
source: myrn summarize
about: sym:bench/src/runner.rs#runner::export_records
anchors:
- sym:bench/src/runner.rs#runner::export_records@16c661c7175b7a25
---
# runner::export_records

Runs myrn --json debug review --sample 0 in work and tags each item with its task, condition and run as origin. Writes the items to a JSON file in dir and returns the count, writing nothing and returning 0 when there are none.

## Connections

- **Location:** `bench/src/runner.rs:460-501`
- **Signature:** `fn export_records( myrn: &Path, work: &Path, home: &Path, dir: &Path, spec: &RunSpec, ) -> Result<usize>`
- **In:** [[file-bench-src-runner-rs]]
- **Calls:** [[condition-as-str]]
- **Called by:** [[runner-run-inner]]
