---
id: seed-find
type: summary
status: unverified
title: seed::find
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::find
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::find@9c3139be42ea0075
map_version: '2'
---
# seed::find

Resolves a query (node id, path:line, file path, qualified name or bare name) to matching nodes, best match first. Read-only, touches only the database.

## Behaviour

- empty, whitespace or only-backtick input -> empty list
- exact node id -> that one node, checked before any other form
- src/a.rs:42 -> smallest symbol or test in that path whose line range contains line 42

## Terms

symbol lookup, resolve name, qualified name, file path, line number, locate definition, best match, case insensitive

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:13-74`
- **Signature:** `pub fn find(db: &Db, query: &str) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-drop-shadowed-stubs]], [[seed-myrn-index-lang]], [[seed-segments]], [[seed-ends-with-segments]], [[graph-node-graph]], [[graph-nodes-where]]
- **Called by:** [[engine-find]], [[seed-seeds]]
