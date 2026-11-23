---
id: record-edgestatus
type: summary
status: unverified
title: record::EdgeStatus
source: myrn summarize
about: sym:crates/myrn-model/src/record.rs#record::EdgeStatus
anchors:
- sym:crates/myrn-model/src/record.rs#record::EdgeStatus@c1582b7011e17aae
map_version: '2'
---
# record::EdgeStatus

Enum that marks whether a derived edge was established with certainty or only guessed. Tree-sitter cannot resolve every call, so guesses are labelled Inferred instead of being presented as fact.

## Behaviour

- Resolved serializes as 'resolved'; Inferred serializes as 'inferred'
- Use Inferred for call edges that tree-sitter could not confirm

## Terms

edge confidence, call resolution, resolved or inferred, guessed edge, edge provenance, derived edge certainty

## Connections

- **Location:** `crates/myrn-model/src/record.rs:41-44`
- **Signature:** `pub enum EdgeStatus`
- **In:** [[file-crates-myrn-model-src-record-rs]]
