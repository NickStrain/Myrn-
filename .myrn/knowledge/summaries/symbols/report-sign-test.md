---
id: report-sign-test
type: summary
status: unverified
title: report::sign_test
source: myrn summarize
about: sym:bench/src/report.rs#report::sign_test
anchors:
- sym:bench/src/report.rs#report::sign_test@eb0f8cf6e2c3562e
map_version: '2'
---
# report::sign_test

Computes the two-sided exact binomial sign test p-value for k successes in n trials at p = 0.5.

## Behaviour

- n = 0 -> None
- k = 5, n = 10 -> Some(1.0), capped at 1.0
- k = 10, n = 10 -> about 0.00195

## Terms

binomial test, p-value, exact test, two-sided, untied pairs, successes out of trials

## Connections

- **Location:** `bench/src/report.rs:54-65`
- **Signature:** `pub fn sign_test(k: usize, n: usize) -> Option<f64>`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-compare-by]]
- **Tests:** `sign_tests`
