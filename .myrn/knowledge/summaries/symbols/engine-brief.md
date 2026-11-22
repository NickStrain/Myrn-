---
id: engine-brief
type: summary
status: unverified
title: Engine::brief
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::brief
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::brief@98f5346fa75b9335
---
# Engine::brief

Builds the session-start brief within the budget (default session_budget): file and symbol counts, the project summary, the top 10 directories by file count, and entry-point files. The text is labelled as reference, not instructions.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:747-807`
- **Signature:** `pub fn brief(&self, budget: Option<usize>) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-status]], [[api-clip-sentences]], [[budget-tokens]], [[budget-new]], [[budget-push]], [[id-module-id]], [[graph-node-graph]], [[graph-nodes-where]]
- **Called by:** [[hook-handle]], [[query-brief]]
- **Tests:** `context_pack_is_relevant_and_bounded`, `packs_are_small_carry_code_and_skip_noise`, `map_is_written_linked_incremental_and_searchable`
