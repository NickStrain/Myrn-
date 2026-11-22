---
id: rank-rank
type: summary
status: unverified
title: rank::rank
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::rank
anchors:
- sym:crates/myrn-engine/src/retrieve/rank.rs#retrieve::rank::rank@e15fcdf41150b891
---
# rank::rank

Consumes seeds and sorts them by score descending. Returns an empty Vec when the top seed is below MIN_SCORE; otherwise returns candidates sorted by score then id, with only read queries against db. Expands only the top 8 seeds scoring at least 3.0, merges duplicate hits as max + 0.25 x min, and adds 0.3 x ln(1 + in_degree) to sym: nodes. Drops any candidate whose final score is not positive.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/rank.rs:22-109`
- **Signature:** `pub fn rank(db: &Db, seeds: Vec<Seed>) -> Result<Vec<Ranked>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-rank-rs]]
- **Calls:** [[rank-record-weight]], [[walks-direct-tests]], [[walks-in-degree]], [[graph-nodes-where]], [[graph-neighbors]]
- **Called by:** [[engine-context-excluding]]
