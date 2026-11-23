---
id: file-crates-myrn-src-cmd-embeddings-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/embeddings.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/embeddings.rs
anchors:
- file:crates/myrn/src/cmd/embeddings.rs@c8b62b97573cc34d
---
# crates/myrn/src/cmd/embeddings.rs

Implements the `myrn embeddings` subcommands (enable, disable, status) that turn optional local semantic search on and off and report its state. Each subcommand gets an engine from the shared `Ctx`, calls one engine method, and prints the result as JSON or plain text.

## Connections

- **Location:** `crates/myrn/src/cmd/embeddings.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[embeddings-enable]], [[embeddings-disable]], [[embeddings-status]]
- **Imports:** [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
