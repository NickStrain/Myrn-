---
id: supported
type: summary
status: unverified
title: supported
source: myrn summarize
about: sym:crates/myrn-embed/src/lib.rs#supported
anchors:
- sym:crates/myrn-embed/src/lib.rs#supported@f03782d9c9470407
map_version: '2'
---
# supported

Reports at compile time whether this build includes the `model` Cargo feature, so the model can run.

## Behaviour

- Built with feature `model` -> true
- Built without feature `model` -> false
- Result is fixed at compile time and does not check files or the network

## Terms

feature flag, build support, compiled in, capability check, model feature, can run model

## Connections

- **Location:** `crates/myrn-embed/src/lib.rs:30-32`
- **Signature:** `pub fn supported() -> bool`
- **In:** [[file-crates-myrn-embed-src-lib-rs]]
- **Called by:** [[client-embed]], [[engine-embeddings-on]], [[engine-enable-embeddings]], [[engine-embeddings-status]]
