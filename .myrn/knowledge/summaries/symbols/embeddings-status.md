---
id: embeddings-status
type: summary
status: unverified
title: embeddings::status
source: myrn summarize
about: sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::status
anchors:
- sym:crates/myrn/src/cmd/embeddings.rs#cmd::embeddings::status@a6a15dca6b5a704d
map_version: '2'
---
# embeddings::status

Reads the engine's embeddings status and reports whether semantic search is enabled, the model name and download state, the model directory, the vector count, and whether the helper process is running. If the build has no embedding support, it prints a single notice instead.

## Behaviour

- s.supported is false -> prints 'This build of myrn has no embedding support.' and nothing else.
- s.enabled is false -> the semantic search line shows 'off' with the hint to run `myrn embeddings enable`.
- s.helper_running is false -> the helper line shows 'not running (starts on first use)'.

## Terms

show state, check embeddings, model installed, vector count, helper process, embedding support, semantic search state

## Connections

- **Location:** `crates/myrn/src/cmd/embeddings.rs:37-54`
- **Signature:** `pub fn status(ctx: &Ctx) -> Result<ExitCode>`
- **In:** [[file-crates-myrn-src-cmd-embeddings-rs]]
- **Calls:** [[engine-embeddings-status]], [[ctx-engine]], [[ctx-emit]]
- **Called by:** [[main-main]]
