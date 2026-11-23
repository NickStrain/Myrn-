---
id: rankingconfig-default
type: summary
status: unverified
title: RankingConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::RankingConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::RankingConfig::default@6884c0a5364216ec
map_version: '2'
---
# RankingConfig::default

Supplies the default ranking settings, also used for any missing key in [ranking].

## Behaviour

- no input -> mode 'learned', usefulness_weight 0.5, freshness_weight 0.2
- no input -> freshness_half_life_days 180.0

## Terms

default ranking, learned default, ranking defaults

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:42-49`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
- **Tests:** `old_records_rank_down_a_little`
