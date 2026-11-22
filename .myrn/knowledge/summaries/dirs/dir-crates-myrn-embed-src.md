---
id: dir-crates-myrn-embed-src
type: summary
status: unverified
title: crates/myrn-embed/src/
source: myrn summarize
about: module:crates/myrn-embed/src
anchors:
- module:crates/myrn-embed/src@e2b7ea978bc40e96
---
# crates/myrn-embed/src/

myrn-embed is an optional embedding helper crate and the only long-lived process in the system, kept off the hot path. lib.rs only declares the public `client` and `helper` submodules; client.rs is the engine-side client that talks to the helper and is documented to fall back cleanly when it is unavailable, and helper.rs is the helper-process entry point that loads a model, serves embeddings, and exits when idle. The provided excerpts contain only doc comments, so the function signatures, types, and protocol details cannot be confirmed from them.

## Connections

- **In:** [[dir-crates-myrn-embed]]
- **Files:** [[file-crates-myrn-embed-src-client-rs]], [[file-crates-myrn-embed-src-helper-rs]], [[file-crates-myrn-embed-src-lib-rs]]
