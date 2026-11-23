---
id: parse-is-constructor-name
type: summary
status: unverified
title: parse::is_constructor_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::is_constructor_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::is_constructor_name@ffbcb10a5c86ed07
map_version: '2'
---
# parse::is_constructor_name

Says whether an associated function name conventionally returns Self, so note_local can treat Type::name() as a construction.

## Behaviour

- new, default, open, create, connect, load, init and build -> true
- new_pool, with_capacity, from_str and open_db -> true by prefix
- renew and build_index -> false; fromage -> true because only the prefix from is checked

## Terms

constructor name, factory name, associated constructor, Self returning, constructor heuristic

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1205-1213`
- **Signature:** `fn is_constructor_name(n: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
