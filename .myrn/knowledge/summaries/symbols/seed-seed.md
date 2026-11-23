---
id: seed-seed
type: summary
status: unverified
title: seed::Seed
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::Seed
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::Seed@b3239cb47431a59f
map_version: '2'
---
# seed::Seed

Holds one starting node: its id, a numeric score (higher is better) and a fixed reason string saying why it was picked.

## Behaviour

- named token hit -> reason named in task or named file, score 10 divided by the square root of the hit count
- word-search hit -> reason matches task words or matching commit, score 6 times relevance plus any summary rank bonus
- the same id can appear more than once in a result list

## Terms

starting node, seed, score, reason, entry point, ranked candidate

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:119-123`
- **Signature:** `pub struct Seed`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
