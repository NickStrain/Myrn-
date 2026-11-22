---
id: check
type: summary
status: unverified
title: check
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#check
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#check@877b931b2d908097
---
# check

Asserts exact set equality of the card's calls, called_by and tests against the Expect fields. Requires every call and caller link to have status 'resolved'. Then asserts impact direct_callers equals called_by, and the union of direct and indirect test ids equals all_tests. Any missing or extra edge fails the test with the symbol id in the message.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:109-135`
- **Signature:** `fn check(e: &Engine, x: &Expect)`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[card]], [[impact]], [[ids]], [[set]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`
