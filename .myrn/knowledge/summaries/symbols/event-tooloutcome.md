---
id: event-tooloutcome
type: summary
status: unverified
title: event::ToolOutcome
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::ToolOutcome
anchors:
- sym:crates/myrn-model/src/event.rs#event::ToolOutcome@db8e7a8cb9a348af
map_version: '2'
---
# event::ToolOutcome

Classifies a tool result as Failure, FileEdit, or Other. It is Copy and Eq, so values compare directly without cloning.

## Behaviour

- "failure" -> Failure; "file_edit" -> FileEdit; "other" -> Other
- Any other string fails deserialization
- The variant names are serialized in snake_case

## Terms

outcome, tool failure, file edit, tool status, success, error classification

## Connections

- **Location:** `crates/myrn-model/src/event.rs:37-41`
- **Signature:** `pub enum ToolOutcome`
- **In:** [[file-crates-myrn-model-src-event-rs]]
