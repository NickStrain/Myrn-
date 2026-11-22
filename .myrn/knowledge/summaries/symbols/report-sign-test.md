---
id: report-sign-test
type: summary
status: unverified
title: report::sign_test
source: myrn summarize
about: sym:bench/src/report.rs#report::sign_test
anchors:
- sym:bench/src/report.rs#report::sign_test@eb0f8cf6e2c3562e
---
# report::sign_test

Two-sided exact binomial sign test of k successes in n trials at p = 0.5, returning None when n is 0 and capping the p-value at 1.0.

## Connections

- **Location:** `bench/src/report.rs:54-65`
- **Signature:** `pub fn sign_test(k: usize, n: usize) -> Option<f64>`
- **In:** [[file-bench-src-report-rs]]
- **Called by:** [[report-compare-by]]
- **Tests:** `sign_tests`
