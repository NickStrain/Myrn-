---
id: relation-parse
type: summary
status: unverified
title: Relation::parse
source: myrn summarize
about: sym:crates/myrn-model/src/relation.rs#relation::Relation::parse
anchors:
- sym:crates/myrn-model/src/relation.rs#relation::Relation::parse@560bbd8609a7d2ab
map_version: '2'
---
# Relation::parse

Looks up a relation by exact, case-sensitive match against as_str over ALL, returning Some on a hit and None otherwise.

## Behaviour

- defined_in -> Some(DefinedIn)
- DefinedIn or depends_on or empty string -> None
- Linear scan of 19 entries; any new variant must also be added to ALL to be found

## Terms

lookup, from string, string to relation, validate, exact match, unknown name, parse relation, closed check

## Connections

- **Location:** `crates/myrn-model/src/relation.rs:77-79`
- **Signature:** `pub fn parse(s: &str) -> Option<Relation>`
- **In:** [[file-crates-myrn-model-src-relation-rs]]
- **Called by:** [[engine-record]], [[frontmatter-parse]]
- **Tests:** `closed_vocabulary`
