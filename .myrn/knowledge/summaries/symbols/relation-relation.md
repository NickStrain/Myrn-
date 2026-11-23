---
id: relation-relation
type: summary
status: unverified
title: relation::Relation
source: myrn summarize
about: sym:crates/myrn-model/src/relation.rs#relation::Relation
anchors:
- sym:crates/myrn-model/src/relation.rs#relation::Relation@1ff9323aa3ec7bf8
map_version: '2'
---
# relation::Relation

Defines the closed set of 19 relation kinds an edge may carry, serialized via serde as snake_case strings. Unknown names are rejected by parse, not by serde.

## Behaviour

- DefinedIn serializes as defined_in and VerifiedBy as verified_by
- ALL is a separate hand-written array; a variant missing from ALL cannot be parsed back from its string
- Copy, Eq and Hash are derived, so variants work as map keys and are passed by value

## Terms

relation type, edge label, link kind, edge type, relation name, closed set, graph edge kind

## Connections

- **Location:** `crates/myrn-model/src/relation.rs:8-28`
- **Signature:** `pub enum Relation`
- **In:** [[file-crates-myrn-model-src-relation-rs]]
