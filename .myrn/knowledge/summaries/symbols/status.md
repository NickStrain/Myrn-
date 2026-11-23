---
id: status
type: summary
status: unverified
title: status
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#status
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#status@b2b8ff508489e1f9
map_version: '2'
---
# status

Looks up the status column of a node by id in the nodes table and returns it, or None if the row is missing or the query fails.

## Behaviour

- id not indexed yet -> None
- indexed id -> Some("verified") or Some("unverified")
- any query error is swallowed and returned as None

## Terms

status lookup, verified, unverified, node status, db query, record state, trust level

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:99-104`
- **Signature:** `fn status(e: &Engine, id: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Calls:** [[engine-db]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `hand_committed_directives_are_stripped_when_shown`, `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`
