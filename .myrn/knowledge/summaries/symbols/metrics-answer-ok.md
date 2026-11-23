---
id: metrics-answer-ok
type: summary
status: unverified
title: metrics::answer_ok
source: myrn summarize
about: sym:bench/src/metrics.rs#metrics::answer_ok
anchors:
- sym:bench/src/metrics.rs#metrics::answer_ok@8d18bc3dc90f9d81
map_version: '2'
---
# metrics::answer_ok

Checks that every expected string appears in the answer as a case-insensitive substring.

## Behaviour

- answer 'It is BinaryDetection::quit', expected ['quit', 'binarydetection'] -> true
- answer 'quit', expected ['convert'] -> false
- Empty expected list -> true (all over nothing)

## Terms

answer check, grading, expected strings, case-insensitive match, substring match, correctness

## Connections

- **Location:** `bench/src/metrics.rs:325-328`
- **Signature:** `pub fn answer_ok(answer: &str, expected: &[String]) -> bool`
- **In:** [[file-bench-src-metrics-rs]]
- **Called by:** [[runner-run-inner]]
- **Tests:** `answers_and_stats`
