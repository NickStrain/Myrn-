---
id: symbolcard-render
type: summary
status: unverified
title: SymbolCard::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render@19a87fc5463f1da0
map_version: '2'
---
# SymbolCard::render

Produces the full multi-line card for one symbol: header, signature, summary, relation lists, tests, changed-in commits, learned records and body hash status. Adds an inferred-marker legend only when some call, caller or test link is inferred.

## Behaviour

- empty or missing signature -> signature row omitted
- no known records -> row reading "no learned records yet"; otherwise at most limit records are shown
- members row only when non-empty; changed-in row shows at most 3 commits and only when non-empty

## Terms

full card, symbol report, detailed view, card text, legend

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:137-191`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-summary-text]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[node-location]]
