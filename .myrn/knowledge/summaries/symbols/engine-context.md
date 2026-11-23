---
id: engine-context
type: summary
status: unverified
title: Engine::context
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::context
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::context@9033852e56d05179
map_version: '2'
---
# Engine::context

Builds a context pack for a task, with no items excluded.

## Behaviour

- Calls context_excluding with an empty seen set
- budget None uses the configured prompt budget

## Terms

task context, prompt pack, relevant code, context for task

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:679-681`
- **Signature:** `pub fn context(&self, task: &str, budget: Option<usize>) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-context-excluding]]
- **Called by:** [[engine-diff-knowledge]], [[order]], [[context-call]], [[query-context]]
- **Tests:** `context_pack_is_relevant_and_bounded`, `packs_are_small_carry_code_and_skip_noise`, `bugs_are_stored_deduplicated_and_recalled`, `published_record_reaches_a_teammate_through_git`, `hand_committed_directives_are_stripped_when_shown`, `map_is_written_linked_incremental_and_searchable`
