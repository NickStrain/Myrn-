---
id: file-crates-myrn-embed-src-client-rs
type: summary
status: unverified
title: crates/myrn-embed/src/client.rs
source: myrn summarize
about: file:crates/myrn-embed/src/client.rs
anchors:
- file:crates/myrn-embed/src/client.rs@e23de2c2af828c68
---
# crates/myrn-embed/src/client.rs

Client for asking for text embeddings: it sends texts to a local helper process over loopback TCP, starts that helper if it is not running, or falls back to loading the model in this process. Every failure returns None so callers can fall back to lexical search.

## Connections

- **Location:** `crates/myrn-embed/src/client.rs`
- **In:** [[dir-crates-myrn-embed-src]]
- **Defines:** [[client-request]], [[client-embed]], [[client-start]], [[client-local]], [[client-local-client]]
- **Imports:** [[file-crates-myrn-embed-src-helper-rs]]
- **Imported by:** [[file-crates-myrn-embed-src-lib-rs]]
