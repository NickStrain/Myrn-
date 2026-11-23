---
id: file-crates-myrn-engine-src-pack-cards-rs
type: summary
status: unverified
title: crates/myrn-engine/src/pack/cards.rs
source: myrn summarize
about: file:crates/myrn-engine/src/pack/cards.rs
anchors:
- file:crates/myrn-engine/src/pack/cards.rs@8cb7ac1eb868d32d
---
# crates/myrn-engine/src/pack/cards.rs

Card builders that turn graph nodes and their relations into the plain text an agent reads: a Link type for related nodes, card structs for symbols, files and change impact, and shared helpers for rows, lists and test summaries. SymbolCard has render (full card) and render_compact (prompt card), and FileCard and ImpactReport each have render.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs`
- **In:** [[dir-crates-myrn-engine-src-pack]]
- **Defines:** [[cards-link]], [[link-new]], [[cards-symbolcard]], [[cards-known]], [[known-line]], [[cards-summary-text]], [[cards-filecard]], [[cards-impactreport]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[symbolcard-render]], [[cards-tests-summary]], [[symbolcard-render-compact]], [[filecard-render]], [[impactreport-render]]
- **Imports:** [[file-crates-myrn-store-src-graph-rs]]
- **Imported by:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-engine-src-pack-mod-rs]], [[file-crates-myrn-engine-tests-gate0-rs]]
