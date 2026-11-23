---
id: file-crates-myrn-engine-tests-notes-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/notes.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/notes.rs
anchors:
- file:crates/myrn-engine/tests/notes.rs@cc9b80a440021f04
---
# crates/myrn-engine/tests/notes.rs

Integration tests for note indexing: they seed a temporary git repo, copy the markdown note fixtures into it, refresh the engine, and check nodes, edges, anchors, summaries, flagged issues, re-indexing, rebuild, and write_note. Helpers at the top build the repo, copy fixtures with placeholder substitution, and read graph state.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write]], [[repo]], [[repo-notes]], [[fixtures]], [[copy-fixtures]], [[copy-fixtures-rec]], [[node]], [[has-edge]], [[body-hash]], [[note-snapshot]], [[indexed-repo]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
