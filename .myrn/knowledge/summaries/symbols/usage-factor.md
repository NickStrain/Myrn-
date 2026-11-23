---
id: usage-factor
type: summary
status: unverified
title: Usage::factor
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::Usage::factor
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::Usage::factor@879d0cd0ad322298
map_version: '2'
---
# Usage::factor

Returns a ranking multiplier clamped to 0.5..1.5, based on the smoothed chance an injected item gets used, raised to the given weight.

## Behaviour

- injected == 0 or weight <= 0 -> 1.0
- used 5, used_good 4, injected 6, weight 0.5 -> about 1.46 (above 1)
- used 0, injected 6, weight 5.0 -> clamped up to 0.5

## Terms

ranking boost, ranking penalty, usefulness score, usefulness weighting, bounded multiplier, smoothed usage rate, prior smoothing

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:40-49`
- **Signature:** `pub fn factor(&self, weight: f64) -> f64`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Called by:** [[rank-learned]]
- **Tests:** `usefulness_moves_with_evidence_and_stays_bounded`, `learned_ranking_moves_items_by_past_use_and_fixed_ignores_it`
