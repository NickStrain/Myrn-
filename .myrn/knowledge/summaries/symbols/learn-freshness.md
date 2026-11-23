---
id: learn-freshness
type: summary
status: unverified
title: learn::freshness
source: myrn summarize
about: sym:crates/myrn-engine/src/learn.rs#learn::freshness
anchors:
- sym:crates/myrn-engine/src/learn.rs#learn::freshness@fe476e0474e5fd5f
map_version: '2'
---
# learn::freshness

Returns a ranking multiplier that drops from 1 toward 1 - freshness_weight as the record's age grows, halving the penalty every freshness_half_life_days.

## Behaviour

- age 0 -> 1.0; negative age is treated as 0 -> 1.0
- default config, age 180 days -> 0.9 (asserted by the test)
- freshness_weight <= 0 or half-life <= 0 -> 1.0 at any age

## Terms

record age, staleness penalty, age decay, half-life, recency, old record demotion, note file time

## Connections

- **Location:** `crates/myrn-engine/src/learn.rs:76-82`
- **Signature:** `pub fn freshness(cfg: &RankingConfig, age_days: f64) -> f64`
- **In:** [[file-crates-myrn-engine-src-learn-rs]]
- **Called by:** [[rank-learned]]
- **Tests:** `old_records_rank_down_a_little`
