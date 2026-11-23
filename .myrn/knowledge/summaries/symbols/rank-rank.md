---
id: rank-rank
type: summary
status: unverified
title: rank::rank
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::rank
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::rank@b27eda3c3808c095
map_version: '2'
---
# rank::rank

Expands the seeds one hop along the graph, merges candidates by id with a diminishing score sum, applies symbol and record weights, and returns candidates with score above 0 sorted by score then id. It only reads the database.

## Behaviour

- Empty seeds, or best seed score below MIN_SCORE (3.0), returns an empty Vec without expanding anything
- Only the first 8 seeds with score at least 3.0 expand; a file: seed adds up to 5 symbols from the same path at 0.5 times its score, reason 'in named file'
- Merge is max(a,b) + 0.25*min(a,b); the reason is replaced only by a strictly higher-scoring contribution; 'calls a match' skips test nodes, 'called by a match' does not

## Terms

context injection, relevance ranking, neighbour expansion, one hop, min score threshold, which symbols shown, retrieval results

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:23-113`
- **Signature:** `pub fn rank(db: &Db, seeds: Vec<Seed>, cfg: &RankingConfig) -> Result<Vec<Ranked>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
- **Calls:** [[rank-learned]], [[rank-record-weight]], [[walks-direct-tests]], [[walks-in-degree]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-context-excluding]], [[viz-map-sort]]
