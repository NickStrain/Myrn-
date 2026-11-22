---
id: card
type: summary
status: unverified
title: card
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#card
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#card@1faa8fcdb5f7a349
---
# card

Calls e.explain(id) and returns the SymbolCard. Panics with the JSON-serialized result if the lookup is not Found(Explained::Symbol).

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:69-77`
- **Signature:** `fn card(e: &Engine, id: &str) -> SymbolCard`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Calls:** [[engine-explain]]
- **Called by:** [[check]]
- **Tests:** `gate0_python`, `gate0_typescript`, `explain_lookup_forms`, `incremental_refresh_tracks_edits_and_deletes`, `summaries_show_on_cards_and_go_stale_with_the_code`
