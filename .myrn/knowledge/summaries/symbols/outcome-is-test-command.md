---
id: outcome-is-test-command
type: summary
status: unverified
title: outcome::is_test_command
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::is_test_command
anchors:
- sym:crates/myrn-engine/src/capture/outcome.rs#capture::outcome::is_test_command@19939fdbe6faeeef
---
# outcome::is_test_command

Splits the command on & ; | newline and ( into segments, drops leading VAR=value words, then checks whether the remaining words start with any entry in RUNNERS on whole-word boundaries, so cargo testify does not match. It does not parse quotes, so a separator inside a quoted string still splits the segment.

## Connections

- **Location:** `crates/myrn-engine/src/capture/outcome.rs:47-58`
- **Signature:** `pub fn is_test_command(command: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-outcome-rs]]
- **Called by:** [[capture-to-entry]], [[checks-test-step]]
- **Tests:** `recognises_test_runs`
