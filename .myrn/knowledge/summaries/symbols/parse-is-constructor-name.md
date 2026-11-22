---
id: parse-is-constructor-name
type: summary
status: unverified
title: parse::is_constructor_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::is_constructor_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::is_constructor_name@ffbcb10a5c86ed07
---
# parse::is_constructor_name

Returns true for names conventionally returning Self, such as new, default, open, create, from*, with_*, and new_*; used to decide whether Type::f() is a construction for local typing.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1205-1213`
- **Signature:** `fn is_constructor_name(n: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
