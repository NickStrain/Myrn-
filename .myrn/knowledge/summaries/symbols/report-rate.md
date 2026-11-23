---
id: report-rate
type: summary
status: unverified
title: report::rate
source: myrn summarize
about: sym:bench/src/report.rs#report::rate
anchors:
- sym:bench/src/report.rs#report::rate@68edd1eb472fcf54
map_version: '2'
---
# report::rate

Returns the fraction of runs in the slice whose success flag is true, or None for an empty slice.

## Behaviour

- [] -> None
- two runs, one successful -> Some(0.5)
- Does not filter errored runs, so callers pass valid runs only

## Terms

success rate, pass fraction, task success, success share

## Connections

- **Location:** `bench/src/report.rs:102-108`
- **Signature:** `fn rate(xs: &[&RunRecord]) -> Option<f64>`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-compare-by]], [[report-render]]
