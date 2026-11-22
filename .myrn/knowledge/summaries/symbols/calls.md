---
id: calls
type: summary
status: unverified
title: calls
source: myrn summarize
about: sym:crates/myrn-engine/tests/summarize.rs#calls
anchors:
- sym:crates/myrn-engine/tests/summarize.rs#calls@0375d18c98c155c8
---
# calls

Returns the number of lines in tmp/calls.log, one per stub model invocation. Returns 0 if the log does not exist yet.

## Connections

- **Location:** `crates/myrn-engine/tests/summarize.rs:51-55`
- **Signature:** `fn calls(tmp: &Path) -> usize`
- **In:** [[file-crates-myrn-engine-tests-summarize-rs]]
- **Tests:** `map_is_written_linked_incremental_and_searchable`
