---
id: setup
type: summary
status: unverified
title: setup
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#setup
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#setup@fa565b82c7e6c539
---
# setup

Creates a fresh tempdir, copies the fixture into repo/, commits it with message 'Add token store and session refresh', and calls Engine::init(root, Some(home)). Asserts the init report has no errors. Each test calls this for an isolated engine, and the tempdir is deleted when the returned Fixture is dropped.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:53-67`
- **Signature:** `fn setup() -> Fixture`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-init]], [[fixture]], [[copy-dir]], [[commit-all]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`, `gate0_whole_graph_is_clean`, `explain_lookup_forms`, `context_pack_is_relevant_and_bounded`
