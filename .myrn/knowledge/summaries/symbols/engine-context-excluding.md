---
id: engine-context-excluding
type: summary
status: unverified
title: Engine::context_excluding
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::context_excluding
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::context_excluding@0b8aaae66d2d145a
map_version: '2'
---
# Engine::context_excluding

Ranks candidates for a task and builds a pack of relevant items, leaving out ids the agent has already seen. It shows code only for confident best matches and one line per item otherwise.

## Behaviour

- No ranked results -> empty pack
- Items must score at least 0.4 times the top score and the minimum score, and the pack is never padded to fill the budget
- Commits appear only for history questions, tests only when the task asks about tests, and module nodes are never shown. Bug reports keep at most 3 items and one card.

## Terms

relevance cutoff, context injection, ranking, prompt budget, seen items, pack rules

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:698-799`
- **Signature:** `pub fn context_excluding( &self, task: &str, budget: Option<usize>, seen: &HashSet<String>, ) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-symbol-card]], [[engine-source-if-short]], [[pack-empty]], [[api-asks-about-history]], [[api-asks-about-tests]], [[engine-vector-seeds]], [[budget-tokens]], [[symbolcard-render-compact]], [[render-pack]], [[rank-rank]], [[seed-drop-shadowed-stubs]], [[seed-seeds]], [[seed-asks-about-bug]], [[graph-node-graph]]
- **Called by:** [[engine-context]], [[hook-handle]]
