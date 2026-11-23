---
id: walker-in-type-scope
type: summary
status: unverified
title: Walker::in_type_scope
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::in_type_scope
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::in_type_scope@8c95ed79519445bb
map_version: '2'
---
# Walker::in_type_scope

Reports whether the innermost scope is a Type container, which decides whether a function is labelled a method.

## Behaviour

- Directly inside impl, class, trait or interface -> true
- A function nested inside a method -> false, because only the innermost scope is checked
- A top-level function -> false

## Terms

inside class, method context, innermost scope, type context, impl scope

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:418-420`
- **Signature:** `fn in_type_scope(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-classify-rust]], [[walker-classify-python]]
