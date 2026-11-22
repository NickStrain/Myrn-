---
id: engine-context-excluding
type: summary
status: unverified
title: Engine::context_excluding
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::context_excluding
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::context_excluding@c6b9370049e9ca8b
---
# Engine::context_excluding

Ranks nodes for a task and keeps those scoring at least 0.4 of the top score (and at least rank::MIN_SCORE), skipping seen ids, tests of matches, modules, commits unless the task mentions history and test files unless it mentions tests. The cutoff is computed before seen ids are removed, so weaker items do not move up; earned records go to known, the first two symbols become code cards (the first gets double the code lines), and the rest are listed under the budget.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:669-733`
- **Signature:** `pub fn context_excluding( &self, task: &str, budget: Option<usize>, seen: &HashSet<String>, ) -> Result<Pack>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-symbol-card]], [[engine-source-if-short]], [[pack-empty]], [[api-asks-about-history]], [[api-asks-about-tests]], [[budget-tokens]], [[symbolcard-render-compact]], [[render-pack]], [[rank-rank]], [[seed-drop-shadowed-stubs]], [[seed-seeds]], [[graph-node-graph]]
- **Called by:** [[engine-context]], [[hook-handle]]
