---
id: file-crates-myrn-embed-src-helper-rs
type: summary
status: unverified
title: crates/myrn-embed/src/helper.rs
source: myrn summarize
about: file:crates/myrn-embed/src/helper.rs
anchors:
- file:crates/myrn-embed/src/helper.rs@74ca22269bff0f7d
---
# crates/myrn-embed/src/helper.rs

Helper process for local embeddings. It loads the model once, serves embedding requests over a localhost TCP port guarded by a per-run token, and exits after an idle period. It also defines the Info, Request and Response types and the helper.json discovery-file path.

## Connections

- **Location:** `crates/myrn-embed/src/helper.rs`
- **In:** [[dir-crates-myrn-embed-src]]
- **Defines:** [[helper-info]], [[helper-request]], [[helper-response]], [[helper-info-path]], [[helper-read-info]], [[helper-serve]], [[helper-serve-helper]], [[helper-answer]], [[helper-token]]
- **Imported by:** [[file-crates-myrn-embed-src-client-rs]], [[file-crates-myrn-embed-src-lib-rs]]
