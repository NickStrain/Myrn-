---
id: record-link
type: summary
status: unverified
title: record::link
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::link
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::link@1863c4b6caf99b04
map_version: '2'
---
# record::link

Builds a Link with the given relation and a copy of the target string, with inverse set to false.

## Behaviour

- link(Relation::About, 'x') -> Link with rel About, target 'x', inverse false

## Terms

edge, relation edge, link builder, target

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:569-575`
- **Signature:** `fn link(rel: Relation, target: &str) -> Link`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Called by:** [[engine-record]]
