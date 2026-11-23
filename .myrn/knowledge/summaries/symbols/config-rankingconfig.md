---
id: config-rankingconfig
type: summary
status: unverified
title: config::RankingConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::RankingConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::RankingConfig@6fd9fd9b76afeb74
map_version: '2'
---
# config::RankingConfig

Ranking settings from [ranking]: the mode plus weights for usefulness and record age.

## Behaviour

- mode 'fixed' -> usefulness and age not applied, only relevance and state
- usefulness_weight 0 -> usefulness has no effect
- freshness_half_life_days 180 -> a record 180 days old has lost half of freshness_weight

## Terms

ranking mode, learned ranking, usefulness boost, record age decay, freshness penalty, half-life

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:29-39`
- **Signature:** `pub struct RankingConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
