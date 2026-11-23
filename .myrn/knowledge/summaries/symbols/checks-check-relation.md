---
id: checks-check-relation
type: summary
status: unverified
title: checks::check_relation
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_relation
anchors:
- sym:crates/myrn-engine/src/distill/checks.rs#distill::checks::check_relation@c9fe51974f680b1d
map_version: '2'
---
# checks::check_relation

Returns an error message if the relation is not in AGENT_RELATIONS, otherwise None.

## Behaviour

- relation in AGENT_RELATIONS -> None
- any other string -> Some message naming the allowed relations
- case-sensitive exact match

## Terms

relation validation, allowed relation, agent relation, relation name

## Connections

- **Location:** `crates/myrn-engine/src/distill/checks.rs:41-48`
- **Signature:** `pub fn check_relation(rel: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-distill-checks-rs]]
- **Called by:** [[engine-record]]
