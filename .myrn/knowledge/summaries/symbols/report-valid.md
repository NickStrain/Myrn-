---
id: report-valid
type: summary
status: unverified
title: report::valid
source: myrn summarize
about: sym:bench/src/report.rs#report::valid
anchors:
- sym:bench/src/report.rs#report::valid@6e9259b7dc317d24
map_version: '2'
---
# report::valid

Returns true when a run has no error, so it counts toward pairs, rates and tables.

## Behaviour

- error None -> true
- error Some(..) -> false, so the run is excluded from pairs and tables
- render counts runs where valid is false as failed to run

## Terms

error filter, failed run, errored run, excluded runs, run validity

## Connections

- **Location:** `bench/src/report.rs:73-75`
- **Signature:** `fn valid(r: &RunRecord) -> bool`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-pairs-of]], [[report-render]]
