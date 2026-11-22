---
id: file-crates-myrn-embed-src-lib-rs
type: summary
status: unverified
title: crates/myrn-embed/src/lib.rs
source: myrn summarize
about: file:crates/myrn-embed/src/lib.rs
anchors:
- file:crates/myrn-embed/src/lib.rs@d53d78fc5c212817
---
# crates/myrn-embed/src/lib.rs

Root module of the myrn-embed crate, an optional embedding helper that is the only long-lived process and sits off the hot path. It declares two public submodules, `client` and `helper`, and contains no logic itself; the model is downloaded only on explicit request and nothing else depends on it.

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs`
- **In:** [[dir-crates-myrn-embed-src]]
- **Imports:** [[file-crates-myrn-embed-src-client-rs]], [[file-crates-myrn-embed-src-helper-rs]]
