---
id: parse-type-name
type: summary
status: unverified
title: parse::type_name
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::type_name
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::type_name@96a5ef56236be2e5
map_version: '2'
---
# parse::type_name

Reduces a type annotation to its bare nominal name, or None when it is not a plain capitalised name.

## Behaviour

- &mut Engine gives Some('Engine'); ': Foo' gives Some('Foo'); crate::a::Db gives Some('Db')
- Vec<T>, Option<T> and Self give None; string gives None because it is lowercase
- Foo | Bar gives None because the name contains non-alphanumeric characters; Foo[] gives Some('Foo')

## Terms

type annotation, nominal type, bare type, type normalisation, capitalised type, type hint

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:1217-1244`
- **Signature:** `fn type_name(raw: &str) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[parse-strip-generics]]
- **Called by:** [[walker-bind-local]]
