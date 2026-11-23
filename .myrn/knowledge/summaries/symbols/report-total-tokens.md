---
id: report-total-tokens
type: summary
status: unverified
title: report::total_tokens
source: myrn summarize
about: sym:bench/src/report.rs#report::total_tokens
anchors:
- sym:bench/src/report.rs#report::total_tokens@3ef810e26ad511dd
map_version: '2'
---
# report::total_tokens

Returns the per-run total token count for the whole session, recording phase included, from session.tokens.total().

## Behaviour

- Returns session.tokens.total() as f64
- Used for the total tokens row and the total measure in paired comparisons

## Terms

session cost, whole session, total tokens, recording included

## Connections

- **Location:** `bench/src/report.rs:116-118`
- **Signature:** `fn total_tokens(r: &RunRecord) -> f64`
- **In:** [[file-bench-src-report-rs]]
- **Calls:** [[tokens-total]]
