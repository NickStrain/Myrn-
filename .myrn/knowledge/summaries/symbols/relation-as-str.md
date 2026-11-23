---
id: relation-as-str
type: summary
status: unverified
title: Relation::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/relation.rs#relation::Relation::as_str
anchors:
- sym:crates/myrn-model/src/relation.rs#relation::Relation::as_str@b60da0812fa42da9
map_version: '2'
---
# Relation::as_str

Returns the snake_case name of a variant as a static string, consuming the value (Relation is Copy). Used as the stored and serialized form.

## Behaviour

- Relation::PartOf -> part_of
- Relation::Related -> related
- Returns &'static str, so no allocation is made

## Terms

name string, string form, text form, wire name, label, serialize, to string

## Connections

- **Location:** `crates/myrn-model/src/relation.rs:53-75`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-relation-rs]]
- **Called by:** [[sync-link]]
