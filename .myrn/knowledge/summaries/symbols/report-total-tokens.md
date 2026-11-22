---
id: report-total-tokens
type: summary
status: unverified
title: report::total_tokens
source: myrn summarize
about: sym:bench/src/report.rs#report::total_tokens
anchors:
- sym:bench/src/report.rs#report::total_tokens@3ef810e26ad511dd
---
# report::total_tokens

Token measure for total session cost, using session.tokens.total() and including the recording phase.

## Connections

- **Location:** `bench/src/report.rs:116-118`
- **Signature:** `fn total_tokens(r: &RunRecord) -> f64`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[tokens-total]]
