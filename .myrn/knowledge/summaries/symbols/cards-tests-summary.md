---
id: cards-tests-summary
type: summary
status: unverified
title: cards::tests_summary
source: myrn summarize
about: sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::tests_summary
anchors:
- sym:crates/myrn-engine/src/pack/cards.rs#pack::cards::tests_summary@72cbfdc37a6bd4ca
map_version: '2'
---
# cards::tests_summary

Summarises a list of test Links as one line counting tests per file, with the most-tested files first and at most three files named.

## Behaviour

- empty list -> "none found"
- file is the location text before the first colon; equal counts sort by file name
- more than 3 files -> ", N more files" appended, even when N is 1

## Terms

test count, tests per file, test coverage, test summary line, grouped tests

## Connections

- **Location:** `crates/myrn-engine/src/pack/cards.rs:195-219`
- **Signature:** `pub fn tests_summary(tests: &[Link]) -> String`
- **In:** [[file-crates-myrn-engine-src-pack-cards-rs]]
- **Called by:** [[symbolcard-render-compact]]
