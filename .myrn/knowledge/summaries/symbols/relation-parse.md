---
id: relation-parse
type: summary
status: unverified
title: Relation::parse
source: myrn summarize
about: sym:crates/myrn-model/src/relation.rs#relation::Relation::parse
anchors:
- sym:crates/myrn-model/src/relation.rs#relation::Relation::parse@560bbd8609a7d2ab
---
# Relation::parse

Returns Some(Relation) when the input exactly matches a variant's as_str() value (case-sensitive, no trimming), otherwise None; it does a linear search over ALL, so callers should treat None as rejection.

## Connections

- **Location:** `crates/myrn-model/src/relation.rs:77-79`
- **Signature:** `pub fn parse(s: &str) -> Option<Relation>`
- **In:** [[file-crates-myrn-model-src-relation-rs]]
- **Called by:** [[engine-record]], [[frontmatter-parse]]
- **Tests:** `closed_vocabulary`
