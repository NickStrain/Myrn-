---
id: report-tokenmeasure
type: summary
status: unverified
title: report::TokenMeasure
source: myrn summarize
about: sym:bench/src/report.rs#report::TokenMeasure
anchors:
- sym:bench/src/report.rs#report::TokenMeasure@05e5682e9c118d6c
map_version: '2'
---
# report::TokenMeasure

Type alias for a per-run token count function, passed by reference to compare_by and the render table.

## Behaviour

- Passed as &tokens or &total_tokens
- Returns f64 for every run, so no run is skipped

## Terms

token function, measure, per-run count, token extractor

## Connections

- **Location:** `bench/src/report.rs:68`
- **Signature:** `type TokenMeasure = dyn Fn(&RunRecord) -> f64;`
- **In:** [[file-bench-src-report-rs]]
