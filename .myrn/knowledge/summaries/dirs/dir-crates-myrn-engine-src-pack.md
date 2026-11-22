---
id: dir-crates-myrn-engine-src-pack
type: summary
status: unverified
title: crates/myrn-engine/src/pack/
source: myrn summarize
about: module:crates/myrn-engine/src/pack
anchors:
- module:crates/myrn-engine/src/pack@b637472c2fce6c4f
---
# crates/myrn-engine/src/pack/

Context-pack assembly for the engine: it packs ranked graph results into a hard token budget in tiers, and renders them as cards, one-line pointers, and quoted learned records. budget.rs supplies the token estimate and limited accumulator, cards.rs builds SymbolCard, FileCard, and ImpactReport with aligned plain-text render methods (render_compact is the prompt-injection form), render.rs assembles the final pack() text, and log.rs writes injection rows to the `usage` table via record() so later outcomes can be attributed to injected notes. mod.rs only declares the budget, cards, log, and render submodules.

## Connections

- **In:** [[dir-crates-myrn-engine-src]]
- **Files:** [[file-crates-myrn-engine-src-pack-budget-rs]], [[file-crates-myrn-engine-src-pack-cards-rs]], [[file-crates-myrn-engine-src-pack-log-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-src-pack-render-rs]]
