---
id: summarize-sym-out
type: summary
status: unverified
title: summarize::sym_out
source: myrn summarize
about: sym:crates/myrn-engine/src/summarize.rs#summarize::sym_out
anchors:
- sym:crates/myrn-engine/src/summarize.rs#summarize::sym_out@77ad0e9e40a5cf74
map_version: '2'
---
# summarize::sym_out

Turns one symbol's JSON value into a SymOut, accepting a plain string or an object with does (or summary), behaviour and terms.

## Behaviour

- 'Returns the suffix.' as a plain string -> does set, behaviour and terms empty
- object with terms but no does or summary -> None
- behaviour keeps at most 3 entries and terms at most 10; empties and case-insensitive duplicates dropped; commas in terms become ';'

## Terms

answer shape, summary fallback, term cleanup, string or object, term commas, dedupe

## Connections

- **Location:** `crates/myrn-engine/src/summarize.rs:642-678`
- **Signature:** `fn sym_out(v: &serde_json::Value) -> Option<SymOut>`
- **In:** [[file-crates-myrn-engine-src-summarize-rs]]
- **Calls:** [[summarize-clean]]
- **Called by:** [[summarize-summarize-file]]
- **Tests:** `symbol_answers_in_both_shapes`
