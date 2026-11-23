---
id: files-validity
type: summary
status: unverified
title: files
source: myrn summarize
about: sym:crates/myrn-engine/tests/validity.rs#files
anchors:
- sym:crates/myrn-engine/tests/validity.rs#files@8e11937bd5b72e0c
map_version: '2'
---
# files

Returns the four fixture files of the sample Python package and its test as (relative path, content) pairs.

## Behaviour

- no args -> 4 pairs: pkg/__init__.py, pkg/recipes.py, pkg/more.py, tests/test_recipes.py
- pkg/recipes.py content is the RECIPES constant
- pkg/more.py defines chunked(it, n) returning [it]

## Terms

fixture, sample package, test fixture files, seed files, sample Python repo

## Connections

- **Location:** `crates/myrn-engine/tests/validity.rs:15-25`
- **Signature:** `fn files() -> Vec<(&'static str, &'static str)>`
- **In:** [[file-crates-myrn-engine-tests-validity-rs]]
- **Called by:** [[repo-48c3]]
- **Tests:** `shared_records_from_a_teammate_apply_when_their_code_matches`
