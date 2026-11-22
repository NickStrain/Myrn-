---
id: engine-context
type: summary
status: unverified
title: Engine::context
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::context
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::context@9033852e56d05179
---
# Engine::context

Returns a context pack for a task with no ids excluded. It is a shortcut for context_excluding with an empty set.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:650-652`
- **Signature:** `pub fn context(&self, task: &str, budget: Option<usize>) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-context-excluding]]
- **Called by:** [[context-call]], [[query-context]]
- **Tests:** `context_pack_is_relevant_and_bounded`, `packs_are_small_carry_code_and_skip_noise`, `bugs_are_stored_deduplicated_and_recalled`, `map_is_written_linked_incremental_and_searchable`
