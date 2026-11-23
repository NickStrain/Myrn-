---
id: engine-suspects
type: summary
status: unverified
title: Engine::suspects
source: myrn summarize
about: sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::suspects
anchors:
- sym:crates/myrn-engine/src/validity/states.rs#validity::states::Engine::suspects@e04fa493ce2cdc92
map_version: '2'
---
# Engine::suspects

Lists suspect records from the drift report, grouping each record's changed anchors under its id.

## Behaviour

- Drift for a record whose status is not suspect (e.g. superseded) is skipped
- Drift whose note is missing from the graph is skipped
- Same note id in several drifts -> one SuspectRecord with multiple changed entries

## Terms

stale records, drift report, outdated notes, code changed

## Connections

- **Location:** `crates/myrn-engine/src/validity/states.rs:57-80`
- **Signature:** `pub fn suspects(&self) -> Result<Vec<SuspectRecord>>`
- **In:** [[file-crates-myrn-engine-src-validity-states-rs]]
- **Calls:** [[engine-changed]], [[graph-node-graph]], [[sync-drift-sync]]
- **Called by:** [[verify-run]]
- **Tests:** `branch_switch_makes_records_suspect_and_valid_again`, `records_from_an_unmerged_branch_apply_only_where_that_code_exists`, `supersede_replaces_and_keeps_history`
