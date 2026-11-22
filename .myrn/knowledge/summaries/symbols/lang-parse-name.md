---
id: lang-parse-name
type: summary
status: unverified
title: Lang::parse_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::parse_name
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::parse_name@e7b86be55a32e6e4
---
# Lang::parse_name

Inverse of name: finds the Lang whose name matches the input exactly (case-sensitive) and returns None if there is no match.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:51-53`
- **Signature:** `pub fn parse_name(s: &str) -> Option<Lang>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[symbols-resolve-all]]
