---
id: engine-log-injection
type: summary
status: unverified
title: Engine::log_injection
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::log_injection
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::log_injection@add5d37a9c8813e9
map_version: '2'
---
# Engine::log_injection

Records which items were injected into a session in the usage log.

## Behaviour

- Delegates to the pack log record function

## Terms

usage logging, injection record, injected items

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:885-887`
- **Signature:** `pub fn log_injection(&self, session: &str, items: &[String]) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[log-record]]
- **Called by:** [[context-call]], [[recall-call]], [[hook-handle]]
- **Tests:** `forget_removes_the_record_and_what_only_it_used`, `gc_merges_copies_drops_stale_rows_and_compacts_old_sessions`, `opening_the_file_or_asking_about_it_counts_as_use`, `sessions_attach_their_outcome`, `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`
