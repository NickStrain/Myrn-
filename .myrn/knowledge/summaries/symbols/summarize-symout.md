---
id: summarize-symout
type: summary
status: unverified
title: summarize::SymOut
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::SymOut
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::SymOut@50e1acff98ede9c9
map_version: '2'
---
# summarize::SymOut

One parsed symbol answer: the does sentence plus behaviour lines and terms, produced only by sym_out.

## Behaviour

- a plain string answer -> behaviour and terms empty
- does can come out empty (from empty input); summarize ignores answers whose does is empty
- derives PartialEq, so a whole answer can be compared by value in tests

## Terms

symbol answer, parsed answer, model answer, behaviour lines

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:634-638`
- **Signature:** `struct SymOut`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
