---
id: expect
type: summary
status: unverified
title: Expect
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#Expect
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#Expect@23eee287a1c7c7e2
map_version: '2'
---
# Expect

Describes the expected answers for one symbol: its calls, callers, direct tests, and all tests that reach it.

## Behaviour

- calls and called_by are the ids expected in explain's call and caller lists
- tests are direct test links from explain
- all_tests is the union of direct and indirect test links from impact

## Terms

expected answers, test expectation, call graph expectation, reference answers

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:100-107`
- **Signature:** `struct Expect<'a>`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
