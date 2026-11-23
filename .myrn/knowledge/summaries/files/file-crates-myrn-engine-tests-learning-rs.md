---
id: file-crates-myrn-engine-tests-learning-rs
type: summary
status: unverified
title: crates/myrn-engine/tests/learning.rs
source: myrn summarize
about: file:crates/myrn-engine/tests/learning.rs
anchors:
- file:crates/myrn-engine/tests/learning.rs@4d23cb5e469b197e
---
# crates/myrn-engine/tests/learning.rs

Integration tests for the learning loop (Step 20): usage marking from injections, session outcome attachment, learned versus fixed ranking, and the no-embeddings case. Its helpers build a temp git repo with two Python functions, read the usage table, and return ranked item ids.

## Connections

- **Location:** `crates/myrn-engine/tests/learning.rs`
- **In:** [[dir-crates-myrn-engine-tests]]
- **Defines:** [[write-learning]], [[repo-learning]], [[used]], [[order]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-store-src-episodes-rs]]
