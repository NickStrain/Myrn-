---
id: parse-action
type: summary
status: unverified
title: parse::Action
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Action
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Action@4de0716c8230df87
map_version: '2'
---
# parse::Action

The decision the walker makes for one AST node: record a definition, push a scope, record a reference, skip the subtree, or descend.

## Behaviour

- Rust use and macro_invocation return Skip after recording imports or macro calls
- JS describe('x', ...) returns a Module scope; it('t', ...) and test('t', ...) return a test def, including with .only, .skip, .concurrent or .todo
- it.each(...) and other property callees fall through to a normal call reference

## Terms

node action, visitor decision, classification, walk instruction, dispatch result

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:142-159`
- **Signature:** `enum Action<'t>`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
