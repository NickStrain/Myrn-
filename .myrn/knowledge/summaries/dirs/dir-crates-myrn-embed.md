---
id: dir-crates-myrn-embed
type: summary
status: unverified
title: crates/myrn-embed/
source: myrn summarize
about: module:crates/myrn-embed
anchors:
- module:crates/myrn-embed@c4eaa057018c7403
---
# crates/myrn-embed/

Optional embedding helper crate and the only long-lived process in the system, kept off the hot path. `src/lib.rs` only declares the public `client` and `helper` submodules; `client.rs` is the engine-side client that talks to the helper and is documented to fall back cleanly when it is unavailable, and `helper.rs` is the helper-process entry point that loads a model, serves embeddings, and exits when idle. The provided excerpts contain only doc comments, so function signatures, types, and the wire protocol cannot be confirmed from them.

## Connections

- **In:** [[dir-crates]]
- **Directories:** [[dir-crates-myrn-embed-src]]
