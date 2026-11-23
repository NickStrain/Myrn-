---
id: runner-validation
type: summary
status: unverified
title: runner::Validation
source: myrn summarize
about: sym:bench/src/runner.rs#runner::Validation
anchors:
- sym:bench/src/runner.rs#runner::Validation@09355ac27982a5fb
map_version: '2'
---
# runner::Validation

Result of validating one task without an agent: whether the check fails on broken code, whether it passes with the fix, and a note with output or an error.

## Behaviour

- Question tasks leave broken_fails and solution_passes as None; note lists the required answer words
- None means that step did not run, for example because setup failed first
- broken_fails true is the desired result: the check failed before the fix

## Terms

task check, verification result, sanity check, validation report, pass fail result

## Connections

- **Location:** `bench/src/runner.rs:710-715`
- **Signature:** `pub struct Validation`
- **In:** [[file-bench-src-runner-rs]]
