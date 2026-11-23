---
id: file-crates-myrn-engine-src-lib-rs
type: summary
status: unverified
title: crates/myrn-engine/src/lib.rs
source: myrn summarize
about: file:crates/myrn-engine/src/lib.rs
anchors:
- file:crates/myrn-engine/src/lib.rs@f69afc0587a394d1
---
# crates/myrn-engine/src/lib.rs

Crate root of the Myrn core engine: declares its public modules and states that all reads and writes from adapters, the MCP server and the CLI must go through the `api` module, which is the only route to the stores. The submodules are `api`, `config`, `capture`, `distill`, `embeddings`, `indexer`, `knowledge`, `learn`, `pack`, `retrieve`, `review`, `sharing`, `summarize`, `validity` and `viz`.

## Connections

- **Location:** `crates/myrn-engine/src/lib.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-embeddings-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-knowledge-rs]], [[file-crates-myrn-engine-src-learn-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-sharing-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-validity-mod-rs]], [[file-crates-myrn-engine-src-viz-rs]]
