---
id: embeddings-enable
type: summary
status: unverified
title: embeddings::enable
source: myrn summarize
about: sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::enable
anchors:
- sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::enable@2dcdb9a98e07fcc3
map_version: '2'
---
# embeddings::enable

Turns semantic search on for the current project by calling the engine's enable_embeddings, streaming its progress lines to stderr unless JSON output is requested. Emits the total and newly embedded vector counts and the elapsed time in seconds.

## Behaviour

- Model not yet downloaded -> enable downloads it, since the module comment says only enable fetches the model.
- ctx.json is true -> progress lines from the engine are suppressed and only the result is emitted.
- Engine creation or enable_embeddings fails -> the error propagates via ? and nothing is emitted.

## Terms

turn on, download model, semantic search, vector index, embed, meaning-based match, progress output, setup

## Connections

- **Location:** `crates/myrn/src/cmd/embeddings.rs:10-27`
- **Signature:** `pub fn enable(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-embeddings-rs]]
- **Calls:** [[engine-enable-embeddings]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
