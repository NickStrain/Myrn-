---
id: file-crates-myrn-engine-src-lib-rs
type: summary
status: unverified
title: crates/myrn-engine/src/lib.rs
source: myrn summarize
about: file:crates/myrn-engine/src/lib.rs
anchors:
- file:crates/myrn-engine/src/lib.rs@d27eb6a46808642e
---
# crates/myrn-engine/src/lib.rs

Crate root for the myrn-engine core. It declares the public modules: `api` (the sole entry point that adapters, the MCP server and the CLI must use for all reads and writes), `config`, and the internal engine modules `capture`, `distill`, `indexer`, `pack`, `retrieve`, `review`, `summarize` and `validity`. It contains no logic of its own.

## Connections

- **Location:** `crates/myrn-engine/src/lib.rs`
- **In:** [[dir-crates-myrn-engine-src]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-capture-mod-rs]], [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-distill-mod-rs]], [[file-crates-myrn-engine-src-indexer-mod-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-retrieve-mod-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-summarize-rs]], [[file-crates-myrn-engine-src-validity-mod-rs]]
