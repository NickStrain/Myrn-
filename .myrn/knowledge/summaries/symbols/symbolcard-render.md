---
id: symbolcard-render
type: summary
status: unverified
title: SymbolCard::render
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::SymbolCard::render@19a87fc5463f1da0
---
# SymbolCard::render

Renders the full multi-line card: header with kind, name, and location, then signature, summary, members, calls, called by, tests, up to three commits, known records capped at limit, and a status line with the indexed body hash. Adds a legend line when any inferred link appears in calls, called_by, or tests.

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:132-186`
- **Signature:** `pub fn render(&self, limit: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Calls:** [[cards-summary-text]], [[cards-row]], [[cards-list]], [[cards-kind-of]], [[node-location]]
