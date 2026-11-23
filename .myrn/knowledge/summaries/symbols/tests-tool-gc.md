---
id: tests-tool-gc
type: summary
status: unverified
title: tests::tool
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::tests::tool
anchors:
- sym:crates/myrn-engine/src/validity/gc.rs#validity::gc::tests::tool@b69b85d1b16bb702
map_version: '2'
---
# tests::tool

Test helper that builds a tool-event Entry with the given time, outcome and file list, agent set to 'claude', and all other fields default.

## Behaviour

- tool(3, 'other', &['a.rs']) -> event 'tool', outcome 'other', files ['a.rs']
- Empty files slice -> files is an empty vec
- test and commits are left at their defaults (None and empty)

## Terms

test helper, fixture, fake entry, mock tool event, sample log line

## Connections

- **Location:** `crates/myrn-engine/src/validity/gc.rs:200-209`
- **Signature:** `fn tool(t: i64, outcome: &str, files: &[&str]) -> Entry`
- **In:** [[file-crates-myrn-engine-src-validity-gc-rs]]
- **Tests:** `compaction_keeps_what_matters`
