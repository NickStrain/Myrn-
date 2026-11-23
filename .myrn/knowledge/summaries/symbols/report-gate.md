---
id: report-gate
type: summary
status: unverified
title: report::Gate
source: myrn summarize
about: sym:bench/src/report.rs#report::Gate
anchors:
- sym:bench/src/report.rs#report::Gate@a7d4987b177f3bdb
map_version: '2'
---
# report::Gate

Result of one paired comparison: verdict, pair count, token difference with interval and percent change, success rates, cheaper-pair count, sign test p-value, and a reason string. It derives Serialize.

## Behaviour

- Option fields are None when there are no pairs or a value cannot be computed
- cheaper counts pairs where the second condition used fewer tokens than the first
- sign_p is computed over untied pairs only

## Terms

comparison result, gate output, paired result, verdict record, sign p-value, reason text

## Connections

- **Location:** `bench/src/report.rs:38-51`
- **Signature:** `pub struct Gate`
- **In:** [[file-bench-src-report-rs]]
