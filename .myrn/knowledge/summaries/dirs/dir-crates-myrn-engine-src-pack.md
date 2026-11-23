---
id: dir-crates-myrn-engine-src-pack
type: summary
status: unverified
title: crates/myrn-engine/src/pack/
source: myrn summarize
about: module:crates/myrn-engine/src/pack
anchors:
- module:crates/myrn-engine/src/pack@8dd3d7878735f33a
---
# crates/myrn-engine/src/pack/

Context packing for the engine: it turns ranked graph nodes into plain-text cards and assembles them into a context pack under a hard token budget. budget.rs supplies the token estimate (characters divided by four, rounded up) and a bounded accumulator, cards.rs builds the symbol, file, and impact-report text, and render.rs assembles the pack. log.rs records which note IDs were injected into a session in the usage table.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]], [[file-crates-myrn-engine-src-pack-log-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-pack-render-rs]]
