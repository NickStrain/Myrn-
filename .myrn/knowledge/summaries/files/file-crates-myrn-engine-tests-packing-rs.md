---
id: file-crates-myrn-engine-tests-packing-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/packing.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/packing.rs
anchors:
- file:crates/myrn-engine/tests/packing.rs@90e1e4909b79e2ed
---
# crates/myrn-engine/tests/packing.rs

Integration tests for context packs and symbol lookup in myrn-engine: they build temporary git repos from inline Python and TypeScript sources, then check that namespace calls through wildcard or export-star re-exports link to tests, that packs stay small with code and no stub or commit noise, and that the brief omits a most-called list. Helpers `write` and `repo` create the fixtures; the tests call `Engine::explain`, `Engine::context`, and `Engine::brief`.

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-packing]], [[repo-packing]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]]
