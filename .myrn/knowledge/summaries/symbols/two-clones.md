---
id: two-clones
type: summary
status: unverified
title: two_clones
source: myrn summarize
about: sym:crates/myrn-engine/tests/sharing.rs#two_clones
anchors:
- sym:crates/myrn-engine/tests/sharing.rs#two_clones@48f92d288804a8c3
map_version: '2'
---
# two_clones

Creates developer A's repo with sample files, commits as Ana, runs Engine::init with home-a, commits the init output as myrn init, then clones it to b and runs Engine::init with home-b. Returns (A, B).

## Behaviour

- tmp dir -> A at tmp/a, B at tmp/b cloned from A
- B's clone includes A's myrn init commit
- each engine has a separate private home, so notes are not shared implicitly

## Terms

two developers, shared repo, origin clone, team setup, developer clone, test fixture, private store

## Connections

- **Location:** `crates/myrn-engine/tests/sharing.rs:74-87`
- **Signature:** `fn two_clones(tmp: &Path) -> (Dev, Dev)`
- **In:** [[file-crates-myrn-engine-tests-sharing-rs]]
- **Calls:** [[engine-init]], [[files]], [[write-sharing]], [[commit-all-sharing]]
- **Tests:** `published_record_reaches_a_teammate_through_git`, `imported_records_start_unverified_whatever_they_say`, `hand_committed_directives_are_stripped_when_shown`, `shared_records_that_disagree_are_flagged_for_a_person`, `a_teammate_verifies_a_shared_record_locally_without_changing_the_file`
