---
id: event-toolresult
type: summary
status: unverified
title: event::ToolResult
source: myrn summarize
about: sym:crates/myrn-model/src/event.rs#event::ToolResult
anchors:
- sym:crates/myrn-model/src/event.rs#event::ToolResult@17b2d71cba1bb6ca
map_version: '2'
---
# event::ToolResult

Payload for one tool call's result: tool name, outcome, touched file paths, and optional shell command or error text.

## Behaviour

- files has no serde default, so it must be present in JSON; an empty array is allowed
- command and error are optional and become None when missing
- files holds absolute or agent-reported paths exactly as given, with no normalization

## Terms

tool call, tool name, touched files, edited paths, shell command, error text

## Connections

- **Location:** `crates/myrn-model/src/event.rs:44-53`
- **Signature:** `pub struct ToolResult`
- **In:** [[file-crates-myrn-model-src-event-rs]]
