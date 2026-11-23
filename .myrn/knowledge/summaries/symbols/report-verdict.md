---
id: report-verdict
type: summary
status: unverified
title: report::Verdict
source: myrn summarize
about: sym:bench/src/report.rs#report::Verdict
anchors:
- sym:bench/src/report.rs#report::Verdict@0f8d28207de96e94
map_version: '2'
---
# report::Verdict

Outcome of a comparison, serialized in snake_case as pass, inconclusive, fail or no_data.

## Behaviour

- Serialized as pass, inconclusive, fail, no_data
- Inconclusive: mean fell but the 95% interval reaches zero or the sign test p is 0.05 or more
- NoData: set by compare_by when no complete pair exists

## Terms

outcome, gate result, pass or fail, snake case, no data state, inconclusive result

## Connections

- **Location:** `bench/src/report.rs:18-24`
- **Signature:** `pub enum Verdict`
- **In:** [[file-bench-src-report-rs]]
