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

Defines the closed vocabulary of relation names used in the graph (an enum of 19 variants) with snake_case string conversion and parsing, so only these relation names are accepted. Includes a unit test confirming every variant round-trips and that unknown names like "depends_on" are rejected.

## Connections

- **Location:** `crates/myrn-model/src/relation.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[relation-relation]], [[relation-as-str]], [[relation-parse]]
- **Imported by:** [[file-crates-myrn-model-src-lib-rs]]
