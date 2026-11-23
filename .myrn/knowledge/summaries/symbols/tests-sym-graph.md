---
id: tests-sym-graph
type: summary
status: unverified
title: tests::sym
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::tests::sym
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::tests::sym@11fef2422de58f25
map_version: '2'
---
# tests::sym

Test helper that builds a Node with the given id and path, type 'symbol' and origin 'derived', and all other fields None.

## Behaviour

- sym('a', 'x.rs') -> id 'a', path Some('x.rs'), ntype 'symbol'
- every Option field other than path -> None
- origin always 'derived'

## Terms

test fixture, helper, stub node, mock node, test node builder

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:203-211`
- **Signature:** `fn sym(id: &str, path: &str) -> Node`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Tests:** `neighbours_both_ways`
