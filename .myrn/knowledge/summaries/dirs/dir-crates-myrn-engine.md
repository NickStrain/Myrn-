---
id: dir-crates-myrn-engine
type: summary
status: unverified
title: crates/myrn-engine/
source: myrn summarize
about: module:crates/myrn-engine
anchors:
- module:crates/myrn-engine@73ea9e41542dff65
---
# crates/myrn-engine/

Core library crate for the myrn code map. `src/api.rs` is the only entry point used by the MCP server and CLI; its `Engine` holds the layout, config, project id and SQLite `Db`, and every read and write goes through it. The other modules in `src/` each own one stage (indexing, retrieval/packing, distill, capture, summarize, review, and a stub validity module), and `tests/` holds per-feature integration tests, with `gate0.rs` exercising the full tool surface against the multi-language fixture in `tests/fixtures/repos/gate0`.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-engine-src]], [[dir-crates-myrn-engine-tests]]
