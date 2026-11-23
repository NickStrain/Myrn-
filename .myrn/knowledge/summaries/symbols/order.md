---
id: order
type: summary
status: unverified
title: order
source: myrn summarize
about: sym:crates/myrn-engine/tests/learning.rs#order
anchors:
- sym:crates/myrn-engine/tests/learning.rs#order@8b3c2e1f0aec2f97
map_version: '2'
---
# order

Calls engine context for a task with no second argument and returns only the ordered item ids. Used by tests to compare ranking before and after history.

## Behaviour

- task "session token handling" -> list of sym ids, best first
- context error -> panics via unwrap
- result changes with config.ranking.mode, learned or fixed

## Terms

ranking, ranked items, context items, result order, learned ranking, fixed ranking

## Connections

- **Location:** `crates/myrn-engine/tests/learning.rs:114-116`
- **Signature:** `fn order(e: &Engine, task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-tests-learning-rs]]
- **Calls:** [[engine-context]]
- **Tests:** `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`
