---
id: engine-brief
type: summary
status: unverified
title: Engine::brief
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::brief
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::brief@98f5346fa75b9335
map_version: '2'
---
# Engine::brief

Builds the short session-start project brief within a budget: file and symbol counts, the project summary, main directories, entry points, and a pointer to the other tools.

## Behaviour

- Budget defaults to the configured session budget
- Lists up to 10 directories by file count, each with a clipped summary
- Entry points are up to 8 files named main.rs, lib.rs, index.ts and similar, shortest path first

## Terms

session start, project overview, onboarding, project summary, start of session

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:813-873`
- **Signature:** `pub fn brief(&self, budget: Option<usize>) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-status]], [[api-clip-sentences]], [[budget-tokens]], [[budget-new]], [[budget-push]], [[id-module-id]], [[graph-node-graph]], [[graph-nodes-where]]
- **Called by:** [[hook-handle]], [[query-brief]]
- **Tests:** `context_pack_is_relevant_and_bounded`, `packs_are_small_carry_code_and_skip_noise`, `map_is_written_linked_incremental_and_searchable`
