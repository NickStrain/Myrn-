---
id: engine-records-request
type: summary
status: unverified
title: Engine::records_request
source: myrn summarize
about: sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::records_request
anchors:
- sym:crates/myrn-engine/src/distill/record.rs#distill::record::Engine::records_request@0dd960ebb96a3124
map_version: '2'
---
# Engine::records_request

Builds the end-of-turn prompt that asks the agent to record what it learned, but only when file edits happened since the last record_request entry. It appends a record_request entry to the episode file as a side effect.

## Behaviour

- continued is true, ask_at_session_end is off, or there is no episode file -> Ok(None), nothing appended
- no file_edit outcomes since the last record_request -> Ok(None), nothing appended
- otherwise the prompt lists sorted, deduplicated edited files, up to 4 symbols without a current summary, up to 3 normalized error signatures, and the last test command

## Terms

session end, stop hook, record prompt, follow-up request, edited files, summary nudge, episode log, record reminder

## Connections

- **Location:** `crates/myrn-engine/src/distill/record.rs:443-543`
- **Signature:** `pub fn records_request(&mut self, session: &str, continued: bool) -> Result<Option<String>>`
- **In:** [[file-crates-myrn-engine-src-distill-record-rs]]
- **Calls:** [[checks-test-step]], [[engine-summary-is-current]], [[episodes-path-for]], [[episodes-append]], [[episodes-read]], [[graph-node-graph]], [[layout-episodes-dir]], [[resolver-exists]]
- **Called by:** [[hook-handle]]
- **Tests:** `session_end_asks_once_per_batch_of_edits`, `shell_edits_are_captured_and_lead_to_a_record_request`
