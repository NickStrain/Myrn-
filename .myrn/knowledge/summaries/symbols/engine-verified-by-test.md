---
id: engine-verified-by-test
type: summary
status: unverified
title: Engine::verified_by_test
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::verified_by_test
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::verified_by_test@1fe8974bb69a0e5d
map_version: '2'
---
# Engine::verified_by_test

Returns a label if a passing run matches a test the record cites through evidence or a verified_by link.

## Behaviour

- No passing runs -> None
- Evidence of kind test_run -> its test step must appear in a passing run
- Match needs a non-empty needle contained in the run's step text

## Terms

cited test, evidence, verified_by link, test match

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:258-293`
- **Signature:** `fn verified_by_test( &self, note: &myrn_store::notes::Note, passes: &[String], ) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[kind-as-str]], [[checks-test-step]], [[engine-test-needles]]
- **Called by:** [[engine-promote]]
