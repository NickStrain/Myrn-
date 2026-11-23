---
id: file-crates-myrn-model-src-node-rs
type: summary
status: unverified
title: crates/myrn-model/src/node.rs
source: myrn summarize
about: file:crates/myrn-model/src/node.rs
anchors:
- file:crates/myrn-model/src/node.rs@b7e58d2d2d95578c
---
# crates/myrn-model/src/node.rs

Defines the node kinds for the code map: the Origin enum (derived or earned), the 12-variant NodeType enum, their snake_case string names, string parsing, and the mapping from each type to its origin. Includes a round-trip unit test that checks every type parses back from its name and that Bug is earned.

## Connections

- **Location:** `crates/myrn-model/src/node.rs`
- **In:** [[dir-crates-myrn-model-src]]
- **Defines:** [[node-origin]], [[node-nodetype]], [[nodetype-as-str]], [[nodetype-parse]], [[nodetype-origin]], [[origin-as-str]]
- **Imported by:** [[file-crates-myrn-model-src-lib-rs]]
