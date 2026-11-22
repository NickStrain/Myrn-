---
id: bash
type: summary
status: unverified
title: bash
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#bash
anchors:
- sym:crates/myrn-engine/tests/records.rs#bash@b333b0395365029c
---
# bash

Constructs an Event with a Bash ToolResult for the given session and command, using outcome Other and empty files with no error.

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:507-520`
- **Signature:** `fn bash(session: &str, command: &str) -> myrn_model::event::Event`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
