---
id: state-state
type: summary
status: unverified
title: state::State
source: myrn summarize
about: sym:crates/myrn-store/src/state.rs#state::State
anchors:
- sym:crates/myrn-store/src/state.rs#state::State@995ec71bece59c5f
map_version: '2'
---
# state::State

Holds the persisted per-project flags: session recording consent, connected agents keyed by adapter name, recent hook timings, and the semantic search (embeddings) switch. Every field has a serde default, so a partial or older state.json still loads.

## Behaviour

- empty JSON object {} -> capture_consent false, consent_time None, agents empty, hook_timings empty, embeddings false
- hook_timings is capped at 50 entries by record_timing, not by the struct itself
- consent_time is only a Unix time; this struct does not set it when capture_consent changes

## Terms

consent, recording consent, capture consent, connected agents, adapter, hook timings, embeddings flag, project state, persisted settings

## Connections

- **Location:** `crates/myrn-store/src/state.rs:11-27`
- **Signature:** `pub struct State`
- **In:** [[file-crates-myrn-store-src-state-rs]]
