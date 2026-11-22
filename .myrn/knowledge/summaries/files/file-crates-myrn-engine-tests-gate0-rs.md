---
id: file-crates-myrn-engine-tests-gate0-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/gate0.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/gate0.rs
anchors:
- file:crates/myrn-engine/tests/gate0.rs@a87a24dd451cab96
---
# crates/myrn-engine/tests/gate0.rs

Gate 0 acceptance test for the engine. It runs explain, impact, context, brief, refresh, history and rebuild against the multi-language fixture at tests/fixtures/repos/gate0 and asserts exact call and test edges. The helpers (setup, check, card, impact) sit above the per-language tests gate0_rust, gate0_python and gate0_typescript, followed by the lookup, context pack and incremental refresh tests.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[fixture]], [[copy-dir]], [[commit-all]], [[fixture-gate0]], [[setup]], [[card]], [[impact]], [[ids]], [[set]], [[expect]], [[check]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]]
