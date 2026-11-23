---
id: calls
type: summary
status: unverified
title: calls
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#calls
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#calls@0375d18c98c155c8
map_version: '2'
---
# calls

Counts how many model invocations were logged by reading the line count of tmp/calls.log.

## Behaviour

- calls.log with 3 lines -> 3
- no calls.log file yet -> 0
- empty calls.log -> 0

## Terms

call count, invocation count, model call count, count log lines, how many runs

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:51-55`
- **Signature:** `fn calls(tmp: &Path) -> usize`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
