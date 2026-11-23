---
id: file-crates-myrn-model-src-relation-rs
type: summary
status: unverified
title: crates/myrn-model/src/relation.rs
source: myrn summarize
about: file:crates/myrn-model/src/relation.rs
anchors:
- file:crates/myrn-model/src/relation.rs@ed828d6ab2920acf
---
# crates/myrn-model/src/relation.rs

Closed vocabulary of graph relation names: the Relation enum with its 19 variants, a hand-maintained ALL array, conversion to and from snake_case strings, and a round-trip test. Nothing outside this list is accepted, so models cannot invent relation names.

## Connections

- **Location:** `crates/myrn-model/src/relation.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[relation-relation]], [[relation-as-str]], [[relation-parse]]
- **Imported by:** [[file-crates-myrn-model-src-lib-rs]]
