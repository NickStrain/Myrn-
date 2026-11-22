---
id: engine-records-request
type: summary
status: unverified
title: Engine::records_request
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::records_request
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::records_request@0dd960ebb96a3124
---
# Engine::records_request

Returns Some prompt text asking the agent to record its learnings, or None when continued is true, the ask_at_session_end config is off, the episode file is missing, or no file_edit entries exist since the last record_request. When it does produce a prompt it has the side effect of appending a record_request entry to the episode log, and it lists the edited files, up to 4 symbols that lack a current summary, up to 3 normalized error signatures and the last test command.

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:432-532`
- **Signature:** `pub fn records_request(&mut self, session: &str, continued: bool) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[checks-test-step]], [[engine-summary-is-current]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[graph-node-graph]], [[layout-episodes-dir]], [[resolver-exists]]
- **Called by:** [[hook-handle]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
