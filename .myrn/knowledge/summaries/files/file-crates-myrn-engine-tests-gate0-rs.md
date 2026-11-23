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

Gate 0 integration tests. They run the engine on the multi-language fixture repo at tests/fixtures/repos/gate0 (Rust, Python, TypeScript) and compare explain and impact answers exactly with known call graphs, along with incremental refresh, lookup forms, and context pack checks. Helpers copy the fixture into a temp git repo, commit it, and build an Engine.

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[fixture]], [[copy-dir]], [[commit-all]], [[fixture-gate0]], [[setup]], [[card]], [[impact]], [[ids]], [[set]], [[expect]], [[check]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]]
