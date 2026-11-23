---
id: embeddings-download-embeddings
type: summary
status: unverified
title: embeddings::download
source: myrn summarize
about: sym:crates/myrn-engine/src/embeddings.rs#embeddings::download#2
anchors:
- sym:crates/myrn-engine/src/embeddings.rs#embeddings::download#2@e7046ee33d4da447
map_version: '2'
---
# embeddings::download

Stub used when the 'embeddings' feature is off. Always returns an error and touches no disk or network.

## Behaviour

- Any input -> Err 'this build of myrn has no embedding support'
- No file or network I/O happens
- Selected at compile time by cfg(not(feature = 'embeddings'))

## Terms

no-op stub, unsupported build, feature off, disabled build, fallback download

## Connections

- **Location:** `crates/myrn-engine/src/embeddings.rs:253-255`
- **Signature:** `fn download(_home: &std::path::Path, _progress: &dyn Fn(&str)) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-embeddings-rs]]
- **Called by:** [[engine-enable-embeddings]]
