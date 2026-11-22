---
id: file-crates-myrn-engine-src-pack-cards-rs
type: summary
status: unverified
title: crates/myrn-engine/src/pack/cards.rs
source: myrn summarize
about: file:crates/myrn-engine/src/pack/cards.rs
anchors:
- file:crates/myrn-engine/src/pack/cards.rs@48b9b0a72ad1ed1f
---
# crates/myrn-engine/src/pack/cards.rs

Builds the compact cards that agents read in place of opening source files: a Link type for relations, SymbolCard, FileCard, and ImpactReport, each with a render method that produces aligned plain-text output. SymbolCard::render_compact is the reduced form used for prompt injection.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs`
- **In:** [[dir-crates-myrn-engine-src-pack]]
- **Defines:** [[cards-link]], [[link-new]], [[cards-symbolcard]], [[cards-known]], [[known-line]], [[cards-summary-text]], [[cards-filecard]], [[cards-impactreport]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[symbolcard-render]], [[cards-tests-summary]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
- **Imports:** [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-tests-gate0-rs]]
