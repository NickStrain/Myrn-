---
id: seed-find
type: summary
status: unverified
title: seed::find
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::find
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::find@9c3139be42ea0075
---
# seed::find

Resolves a query (node id, `path:line`, file path, qualified or bare name, trimmed of backticks and trailing `()`) to nodes in tried order, returning the first non-empty match set; name matches fall back to case-insensitive, filter to those whose qualname ends with the query's segments, drop shadowed stubs, and sort exact-qualname first, code before tests, then shallower and by path.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:13-74`
- **Signature:** `pub fn find(db: &Db, query: &str) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-drop-shadowed-stubs]], [[seed-myrn-index-lang]], [[seed-segments]], [[seed-ends-with-segments]], [[graph-node-graph]], [[graph-nodes-where]]
- **Called by:** [[engine-find]], [[seed-seeds]]
