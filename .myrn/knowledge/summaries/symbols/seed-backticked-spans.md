---
id: seed-backticked-spans
type: summary
status: unverified
title: seed::backticked_spans
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked_spans
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::backticked_spans@b2481ed91b595675
map_version: '2'
---
# seed::backticked_spans

Returns the text between each pair of backticks, in order, without the backticks.

## Behaviour

- use `a` and `b` -> [a, b]
- an unclosed trailing backtick still yields the text after it
- no backticks -> empty list

## Terms

backtick, code span, inline code, literal text

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:159-165`
- **Signature:** `fn backticked_spans(task: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Called by:** [[seed-task-words]]
