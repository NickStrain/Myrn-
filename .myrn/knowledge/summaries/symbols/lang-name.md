---
id: lang-name
type: summary
status: unverified
title: Lang::name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::name
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::name@0137e45fc4908864
map_version: '2'
---
# Lang::name

Returns the lowercase language id string for the variant.

## Behaviour

- Rust -> rust
- Tsx -> tsx
- JavaScript -> javascript

## Terms

language id, identifier string, string form, lowercase label, serialized name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:41-49`
- **Signature:** `pub fn name(self) -> &'static str`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[indexer-persist]]
