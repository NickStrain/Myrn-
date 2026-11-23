---
id: fact
type: summary
status: unverified
title: fact
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#fact
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#fact@28455bb3cc67c40b
map_version: '2'
---
# fact

Records a fact with body, about, and evidence pointing at the about symbol, asserts it was accepted, and returns the first new id. A rejected record panics with the render output.

## Behaviour

- body and about -> id string of the new fact
- rejected record -> panic showing the rejection
- only ids[0] is returned, so multi-id outcomes lose the rest

## Terms

record fact, add fact, evidence, store fact, record_json, new fact id

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:89-97`
- **Signature:** `fn fact(e: &mut Engine, body: &str, about: &str) -> String`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Calls:** [[engine-record-json]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `shared_records_that_disagree_are_flagged_for_a_person`
