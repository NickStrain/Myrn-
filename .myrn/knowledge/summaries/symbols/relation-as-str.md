---
id: relation-as-str
type: summary
status: unverified
title: Relation::as_str
source: myrn summarize
about: sym:crates/myrn-model/src/relation.rs#relation::Relation::as_str
anchors:
- sym:crates/myrn-model/src/relation.rs#relation::Relation::as_str@b60da0812fa42da9
---
# Relation::as_str

Returns the canonical snake_case string for a variant (e.g. Calls to "calls", ImplementedBy to "implemented_by"); this match must be kept in sync with the enum and the ALL array, since parse depends on it.

## Connections

- **Location:** `crates/myrn-model/src/relation.rs:53-75`
- **Signature:** `pub fn as_str(self) -> &'static str`
- **In:** [[file-crates-myrn-model-src-relation-rs]]
- **Called by:** [[sync-link]]
