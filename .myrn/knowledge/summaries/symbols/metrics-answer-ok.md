---
id: metrics-answer-ok
type: summary
status: unverified
title: metrics::answer_ok
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::answer_ok
anchors:
- sym:bench/src/metrics.rs#metrics::answer_ok@8d18bc3dc90f9d81
---
# metrics::answer_ok

Returns true only if every expected string appears in the answer, compared case-insensitively.

## Connections

- **Location:** `bench/src/metrics.rs:325-328`
- **Signature:** `pub fn answer_ok(answer: &str, expected: &[String]) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[runner-run-inner]]
- **Tests:** `answers_and_stats`
