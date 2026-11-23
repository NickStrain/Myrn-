---
id: cards-symbolcard
type: summary
status: unverified
title: cards::SymbolCard
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard@205203c71259a207
map_version: '2'
---
# cards::SymbolCard

Holds everything shown for one symbol: its node, calls, callers, members, tests, commits, learned records, and whether its summary was written for an older body. render and render_compact turn it into text.

## Behaviour

- summary_outdated true -> summary is shown with a 'possibly outdated' prefix
- empty lists are allowed; render then prints 'none found' for calls, called by and tests
- known holds Known records; render shows at most limit of them

## Terms

symbol card, callers and callees, learned facts, stale summary, per-symbol view

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:35-46`
- **Signature:** `pub struct SymbolCard`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
