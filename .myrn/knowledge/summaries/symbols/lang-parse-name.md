---
id: lang-parse-name
type: summary
status: unverified
title: Lang::parse_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::parse_name
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::parse_name@e7b86be55a32e6e4
map_version: '2'
---
# Lang::parse_name

Inverse of name: returns the Lang whose name equals the input exactly. The match is case-sensitive.

## Behaviour

- tsx -> Some(Tsx)
- Rust -> None, because matching is case-sensitive
- empty string -> None

## Terms

from string, lookup, language id, case sensitive, parse, string to language

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:51-53`
- **Signature:** `pub fn parse_name(s: &str) -> Option<Lang>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[symbols-resolve-all]]
