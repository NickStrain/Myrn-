---
id: file-crates-myrn-engine-tests-notes-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/notes.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/notes.rs
anchors:
- file:crates/myrn-engine/tests/notes.rs@ada26dd55d5ebccc
---
# crates/myrn-engine/tests/notes.rs

Integration tests for Step 15 markdown note indexing. Builds a temp git repo from the PY sample files, copies the notes fixtures into it, and checks that notes become nodes, edges, anchors and summaries, that hand edits and deletions re-index, that malformed or dangling notes are flagged, that rebuild reproduces the index, and that write_note writes files and handles shared versus private duplicates.

## Connections

- **Location:** `crates/myrn-engine/tests/notes.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write]], [[repo]], [[repo-notes]], [[fixtures]], [[copy-fixtures]], [[copy-fixtures-rec]], [[node]], [[has-edge]], [[body-hash]], [[note-snapshot]], [[indexed-repo]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-model-src-lib-rs]], [[file-crates-myrn-store-src-graph-rs]], [[file-crates-myrn-store-src-notes-mod-rs]]
