---
id: dir-crates-myrn-engine
type: summary
status: unverified
title: crates/myrn-engine/
source: myrn summarize
about: module:crates/myrn-engine
anchors:
- module:crates/myrn-engine@5932bba3ba5dd965
---
# crates/myrn-engine/

Core Rust crate for the Myrn code-context tool. src/ holds the engine: indexer/ builds a SQLite code graph from the working tree and git history; capture/ and distill/ turn agent sessions into validated knowledge records; retrieve/, pack/ and validity/ use that graph and those records to find, assemble and check context. Adapters must go through api.rs, the only code touching the stores, while config.rs, learn.rs, embeddings.rs, summarize.rs and review.rs provide settings, ranking feedback, semantic seeds, summaries and hand review. tests/ has per-feature integration tests via the public Engine API, and examples/ holds manual debug binaries such as simcheck.rs.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-engine-examples]], [[dir-crates-myrn-engine-src]], [[dir-crates-myrn-engine-tests]]
