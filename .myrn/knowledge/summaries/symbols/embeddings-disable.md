---
id: embeddings-disable
type: summary
status: unverified
title: embeddings::disable
source: myrn summarize
about: sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::disable
anchors:
- sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::disable@4aa1462dada98e9f
map_version: '2'
---
# embeddings::disable

Turns semantic search off for the current project by calling the engine's disable_embeddings, which drops the stored vectors. Emits {"enabled": false} or a text notice.

## Behaviour

- Semantic search on -> vectors are dropped and output reports it is off.
- Model files are not deleted, so they remain available for other projects.
- Engine error from ctx.engine() or disable_embeddings -> propagated as Err, nothing emitted.

## Terms

turn off, drop vectors, remove embeddings, clear index, keep model, semantic search off

## Connections

- **Location:** `crates/myrn/src/cmd/embeddings.rs:29-35`
- **Signature:** `pub fn disable(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-embeddings-rs]]
- **Calls:** [[engine-disable-embeddings]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
