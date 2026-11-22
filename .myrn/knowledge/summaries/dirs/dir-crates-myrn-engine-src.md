---
id: dir-crates-myrn-engine-src
type: summary
status: unverified
title: crates/myrn-engine/src/
source: myrn summarize
about: module:crates/myrn-engine/src
anchors:
- module:crates/myrn-engine/src@9ac228ca8cd5caa0
---
# crates/myrn-engine/src/

The engine crate's core, organized around `api.rs`, the sole entry point for adapters, the MCP server and the CLI: `Engine` owns the layout, config, project id and SQLite `Db`, and all reads and writes go through it. Internal modules each own one stage: `indexer` builds the code and git graph, `retrieve` and `pack` answer queries within a token budget, `distill` stores agent-reported records, `capture` logs consented session episodes, `summarize` writes model-generated map notes, `review` runs the hand-checked precision gate, and `validity` is still a stub for record state and GC.

## Connections

- **In:** [[dir-crates-myrn-engine]]
- **Directories:** [[dir-crates-myrn-engine-src-capture]], [[dir-crates-myrn-engine-src-distill]], [[dir-crates-myrn-engine-src-indexer]], [[dir-crates-myrn-engine-src-pack]], [[dir-crates-myrn-engine-src-retrieve]], [[dir-crates-myrn-engine-src-validity]]
- **Files:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-config-rs]], [[file-crates-myrn-engine-src-lib-rs]], [[file-crates-myrn-engine-src-review-rs]], [[file-crates-myrn-engine-src-summarize-rs]]
