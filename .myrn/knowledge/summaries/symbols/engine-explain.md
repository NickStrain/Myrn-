---
id: engine-explain
type: summary
status: unverified
title: Engine::explain
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::explain
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::explain@0734598cd9e4146c
map_version: '2'
---
# Engine::explain

Resolves a name and returns a symbol or file card, or the ambiguous or not-found result.

## Behaviour

- File node -> Explained::File
- Any other node -> Explained::Symbol

## Terms

describe, what is this, summary card, explain symbol

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:522-531`
- **Signature:** `pub fn explain(&self, query: &str) -> Result<Lookup<Explained>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-pick]], [[engine-symbol-card]], [[engine-file-card]]
- **Called by:** [[card]], [[explain-call]], [[query-explain]], [[ui-serve]]
- **Tests:** `explain_lookup_forms`, `python_namespace_calls_through_wildcard_reexports_link_tests`, `typescript_namespace_calls_through_export_star_link_tests`, `bugs_are_stored_deduplicated_and_recalled`, `contradictions_are_flagged_and_superseded_records_drop_out`, `summaries_show_on_cards_and_go_stale_with_the_code`
