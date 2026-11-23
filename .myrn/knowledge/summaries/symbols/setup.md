---
id: setup
type: summary
status: unverified
title: setup
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#setup
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#setup@fa565b82c7e6c539
map_version: '2'
---
# setup

Creates a temp dir, copies the gate0 fixture into it as repo, commits it with the message 'Add token store and session refresh', and initialises an Engine with a separate home dir.

## Behaviour

- each call -> a fresh temp directory, so tests do not share state
- Engine::init report with any errors -> assertion failure with the error list
- returns a Fixture whose root is the committed repo

## Terms

test setup, temp repo, init engine, fresh index, test fixture

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:53-67`
- **Signature:** `fn setup() -> Fixture`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-init]], [[fixture]], [[copy-dir]], [[commit-all]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`, `gate0_whole_graph_is_clean`, `explain_lookup_forms`, `context_pack_is_relevant_and_bounded`
