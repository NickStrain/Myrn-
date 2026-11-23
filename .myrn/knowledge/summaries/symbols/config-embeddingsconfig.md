---
id: config-embeddingsconfig
type: summary
status: unverified
title: config::EmbeddingsConfig
source: myrn summarize
about: sym:crates/myrn-engine/src/config.rs#config::EmbeddingsConfig
anchors:
- sym:crates/myrn-engine/src/config.rs#config::EmbeddingsConfig@719c7b1d28c570ad
map_version: '2'
---
# config::EmbeddingsConfig

Tuning for semantic vector search under [embeddings]. Turning it on per machine is done by myrn embeddings enable, not by this struct.

## Behaviour

- similarity below min_similarity (0.40) -> not a vector match
- a match must also beat the task's 20th-best similarity by margin (0.08)
- top_k 5 -> at most 5 vector matches per task

## Terms

semantic search, vector match, cosine similarity threshold, background margin, helper idle timeout, top matches

## Connections

- **Location:** `crates/myrn-engine/src/config.rs:56-67`
- **Signature:** `pub struct EmbeddingsConfig`
- **In:** [[file-crates-myrn-engine-src-config-rs]]
