---
id: check
type: summary
status: unverified
title: check
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#check
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#check@877b931b2d908097
map_version: '2'
---
# check

Asserts that explain and impact for one symbol match an Expect exactly, and that every call and caller link has status resolved.

## Behaviour

- calls, callers, and tests from explain must equal the expected sets with no extras
- any call or caller link with a status other than resolved, including None -> assertion failure
- impact direct_callers must equal called_by, and direct plus indirect tests must equal all_tests

## Terms

assert answers, compare graph, exact match, resolution check, gate check

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:109-135`
- **Signature:** `fn check(e: &Engine, x: &Expect)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[card]], [[impact]], [[ids]], [[set]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`
