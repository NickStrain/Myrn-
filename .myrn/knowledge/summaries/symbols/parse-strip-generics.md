---
id: parse-strip-generics
type: summary
status: unverified
title: parse::strip_generics
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::strip_generics
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::strip_generics@834dd07a42cad629
map_version: '2'
---
# parse::strip_generics

Removes everything inside angle brackets, including nested ones, and any leading & from a type string.

## Behaviour

- Vec<T> gives 'Vec'; a::B<T, U> gives 'a::B'; Map<K, Vec<V>> gives 'Map'
- &Foo<Bar> gives 'Foo'
- A stray > drives depth negative, so a>b gives 'a'; an unclosed < drops the rest

## Terms

generic arguments, angle brackets, type parameters, drop generics, base type name

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1247-1259`
- **Signature:** `fn strip_generics(s: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Called by:** [[walker-rust-type-name]], [[parse-type-name]]
