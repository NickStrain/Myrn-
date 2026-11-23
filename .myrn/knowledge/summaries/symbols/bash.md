---
id: bash
type: summary
status: unverified
title: bash
source: myrn summarize
about: sym:crates/myrn-engine/tests/records.rs#bash
anchors:
- sym:crates/myrn-engine/tests/records.rs#bash@b333b0395365029c
map_version: '2'
---
# bash

Builds a synthetic Bash tool-result event for the given session and command, with outcome Other and no files or error. It is used to feed shell commands into capture.

## Behaviour

- session 's3' and a sed command -> Bash ToolResult event carrying that command
- outcome is always ToolOutcome::Other, files empty, error None
- cwd is always None

## Terms

shell command event, bash tool result, fake hook event, shell capture input, synthetic event, command record

## Connections

- **Location:** `crates/myrn-engine/tests/records.rs:507-520`
- **Signature:** `fn bash(session: &str, command: &str) -> myrn_model::event::Event`
- **In:** [[file-crates-myrn-engine-tests-records-rs]]
- **Tests:** `shell_edits_are_captured_and_lead_to_a_record_request`
