---
id: embeddingsconfig-default
type: summary
status: unverified
title: EmbeddingsConfig::default
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::EmbeddingsConfig::default
anchors:
- sym:crates/myrn-engine/src/config.rs#config::EmbeddingsConfig::default@b5b84dfeb576c0ff
map_version: '2'
---
# EmbeddingsConfig::default

Supplies the built-in embedding thresholds, match limit and helper idle timeout.

## Behaviour

- no input -> min_similarity 0.40, margin 0.08
- no input -> top_k 5, idle_secs 600

## Terms

embedding defaults, default similarity threshold, default top_k

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:70-77`
- **Signature:** `fn default() -> Self`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
