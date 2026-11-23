---
id: card
type: summary
status: unverified
title: card
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#card
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#card@1faa8fcdb5f7a349
map_version: '2'
---
# card

Calls engine explain on an id and returns the SymbolCard, panicking if the result is not a found symbol.

## Behaviour

- Found(Explained::Symbol) -> the SymbolCard
- NotFound, Ambiguous, or a File result -> panic with the JSON of the result
- id is a symbol id such as sym:path#qualified::name

## Terms

symbol card, explain lookup, unwrap card, symbol details

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:69-77`
- **Signature:** `fn card(e: &Engine, id: &str) -> SymbolCard`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-explain]]
- **Called by:** [[check]]
- **Tests:** `gate0_python`, `gate0_typescript`, `explain_lookup_forms`, `incremental_refresh_tracks_edits_and_deletes`, `summaries_show_on_cards_and_go_stale_with_the_code`
